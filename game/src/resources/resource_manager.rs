use std::collections::HashMap;
use std::{fs, mem};
use std::future::Future;
use std::path::PathBuf;
use std::process::Output;
use std::sync::{Arc, Mutex};
use anyhow::Error;
use json::JsonValue;
use json::object::Object;
use tokio::task::{JoinError, JoinSet};
use crate::{error, TaskManager};
use crate::resources::content_pack::ContentPack;

pub struct ResourceManager {
    //Instantiators
    instantiators: HashMap<String, fn(&mut ResourceManager, &Object) -> Result<Option<Box<dyn NamedType>>, Error>>,
    //Map of types and named types of that type
    types: HashMap<String, Vec<String>>,
    //Map of types to their name
    named_types: HashMap<String, Box<dyn NamedType>>
}

impl ResourceManager {
    pub fn new() -> Self {
        return ResourceManager {
            instantiators: HashMap::new(),
            types: HashMap::new(),
            named_types: HashMap::new()
        }
    }

    pub fn get_type(&self, name: &str) -> Option<&Box<dyn NamedType>> {
        return self.named_types.get(name);
    }

    pub fn get_all_of_type(&self, obj_type: &str) -> Option<&Vec<String>>{
        return self.types.get(obj_type);
    }

    pub async fn load_all(&mut self) {
        //for json in loading.types() {
            //let loader = task_manager.get_runtime(true).spawn(Self::load_json(json));
            //task_manager.get_runtime(false).spawn(Self::load_types(loader));
        //}
    }

    async fn load_json(path: PathBuf) -> Result<JsonValue, Error> {
        return Ok(json::parse(String::from_utf8(fs::read(path)?)?.as_str())?);
    }

    async fn load_types(loading: impl Future<Output=Result<Result<JsonValue, Error>, JoinError>>) {
        let found = match loading.await {
            Ok(value) => match value {
                Ok(value) => value,
                Err(error) => {
                    error!("Error loading JSON: {}", error);
                    return;
                }
            },
            Err(error) => {
                error!("Error joining thread: {}", error);
                return;
            }
        };
        let mut types = Self::get_types(found);

        let mut removing: Vec<usize> = Vec::new();
        while !types.is_empty() {
            let mut i = 0;
            for found in &types {
                /*match Self::load_value(found) {
                    Ok(optional) => {
                        if optional.is_some() {
                            removing.push(i);
                        }
                    }
                    Err(error) => error!("Error loading JSON:\n{}", error)
                }
                i += 1;*/
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

    fn get_types(found: JsonValue) -> Vec<Object> {
        let mut types = Vec::new();
        match found {
            JsonValue::Array(values) => {
                for value in values {
                    match value {
                        JsonValue::Object(object) => types.push(object),
                        _ => {
                            error!("Tried to load JSON with unknown top level object: {}", value);
                            continue
                        }
                    }
                }
            },
            JsonValue::Object(object) => types.push(object),
            _ => {
                error!("Tried to load JSON with unknown top level object: {}", found);
                return Vec::new();
            }
        }
        return types;
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