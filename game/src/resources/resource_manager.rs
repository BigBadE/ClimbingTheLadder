use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Error;
use json::JsonValue;
use json::object::Object;
use crate::{error, Game};
use crate::resources::content_pack::ContentPack;
use crate::resources::ContentLoader;
use crate::util::alloc_handle::AllocHandle;

pub struct ResourceManager {
    //Instantiators
    instantiators: HashMap<String, fn(&mut ResourceManager, &Object) -> Result<Option<Arc<dyn NamedType + Send + Sync>>, Error>>,
    //Map of types and named types of that type
    types: HashMap<String, Vec<String>>,
    //Map of types to their name
    named_types: HashMap<String, Arc<dyn NamedType + Send + Sync>>
}

impl ResourceManager {
    pub fn new() -> Self {
        return ResourceManager {
            instantiators: HashMap::new(),
            types: HashMap::new(),
            named_types: HashMap::new()
        }
    }

    pub async fn load_types(content: Box<dyn ContentLoader + Send>) -> AllocHandle {
        let mut content = content.load_main_content();
        return AllocHandle::new(&mut content);
    }

    pub fn finish_loading(game: &mut Game, handle: &AllocHandle) {
        let content = handle.read::<ContentPack>();
    }

    pub fn get_type(&self, name: &str) -> Option<&Arc<dyn NamedType + Send + Sync>> {
        return self.named_types.get(name);
    }

    pub fn get_all_of_type(&self, obj_type: &str) -> Option<&Vec<String>>{
        return self.types.get(obj_type);
    }

    pub fn load_all(&mut self, loading: Vec<JsonValue>) {
        let loading = JsonValue::Array(loading);
        let mut types = ResourceManager::flatten(&loading);

        let mut removing = Vec::new();
        while !types.is_empty() {
            let mut i = 0;
            for found in &types {
                match self.load_value(found) {
                    Ok(optional) => {
                        if optional.is_some() {
                            removing.push(i);
                        }
                    }
                    Err(error) => error!("Error loading JSON:\n{}", error)
                }
                i += 1;
            }
            if removing.is_empty() {
                for value in &types {
                    error!("Couldn't load JSON values, check all the NamedTypes exist:\n{}", value.dump())
                }
            }
            for value in &removing {
                types.remove(*value as usize);
            }
        }
    }

    pub fn flatten(value: &JsonValue) -> Vec<&Object> {
        return match value {
            JsonValue::Array(values) => {
                let mut array = vec!();
                for value in values {
                    for element in ResourceManager::flatten(value) {
                        array.push(element);
                    }
                }
                array
            },
            JsonValue::Object(object) => vec!(object),
            _ => {
                error!("Unknown parent type in JSON\n{}", value);
                vec!()
            }
        }
    }

    fn load_value(&mut self, value: &Object) -> Result<Option<()>, Error> {
        let obj_type = match value.get("$type") {
            Some(name) => name,
            None => {
                return Err(Error::msg(format!("No name for parent type in JSON\n{}", value.dump())));
            }
        }.dump();
        let obj_type = obj_type.as_str();

        let creator = match self.instantiators.get(obj_type) {
            Some(instantiator) => match instantiator(self, value) {
                Ok(value) => match value {
                    Some(creator) => creator,
                    None => return Ok(None)
                },
                Err(error) => return Err(error)
            },
            None => {
                return Err(Error::msg(format!("No name for parent type in JSON\n{}", value.dump())));
            }
        };

        let name = match value.get("$name") {
            Some(name) => name,
            None => {
                return Err(Error::msg(format!("No name given for type {}", value.dump())));
            }
        }.dump();
        let name = name;

        if self.named_types.contains_key(name.as_str()) {
            return Err(Error::msg(format!("Non-unique name {}", name)));
        } else {
            self.named_types.insert(name.to_string(), creator);
        }

        match self.types.get_mut(obj_type) {
            Some(list) => list.push(name),
            None => {
                self.types.insert(name.to_string(), vec!(name));
            }
        }

        return Ok(Some(()));
    }
}

pub trait NamedType {
    fn name(&self) -> String;
}