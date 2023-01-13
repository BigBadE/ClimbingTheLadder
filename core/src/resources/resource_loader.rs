use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use json::JsonValue;
use json::object::Object;
use crate::error;

pub struct ResourceLoader {
    //Instantiators
    instantiators: HashMap<String, fn(&Object) -> Rc<dyn NamedType>>,
    //Map of types and named types of that type
    types: HashMap<String, Vec<String>>,
    //Map of types to their name
    named_types: HashMap<String, Rc<dyn NamedType>>
}

impl ResourceLoader {
    pub fn new() -> Self {
        return ResourceLoader {
            instantiators: HashMap::new(),
            types: HashMap::new(),
            named_types: HashMap::new()
        }
    }

    pub fn get_type(&self, name: &str) -> Option<&Rc<dyn NamedType>> {
        return self.named_types.get(name);
    }

    pub fn get_all_of_type(&self, obj_type: &str) -> Option<&Vec<String>>{
        return self.types.get(obj_type);
    }

    pub fn load(&mut self, value: &JsonValue) {
        match value {
            JsonValue::Array(values) => {
                for value in values {
                    self.load(value)
                }
            },
            JsonValue::Object(object) => self.load_value(object),
            _ => {
                error!("Unknown parent type in JSON\n{}", value);
            }
        }
    }

    fn load_value(&mut self, value: &Object) {
        let obj_type = match value.get("$type") {
            Some(name) => name,
            None => {
                error!("No name for parent type in JSON\n{}", value.dump());
                return;
            }
        }.dump();
        let obj_type = obj_type.as_str();

        let creator = match self.instantiators.get(obj_type) {
            Some(instantiator) => instantiator(value),
            None => {
                error!("No name for parent type in JSON\n{}", value.dump());
                return;
            }
        };

        let name = match value.get("$name") {
            Some(name) => name,
            None => {
                error!("No name given for type {}", value.dump());
                return;
            }
        }.dump();
        let name = name;

        if self.named_types.contains_key(name.as_str()) {
            error!("Non-unique name {}", name);
            return;
        } else {
            self.named_types.insert(name.to_string(), creator);
        }

        match self.types.get_mut(obj_type) {
            Some(list) => list.push(name),
            None => {
                self.types.insert(name.to_string(), vec!(name));
            }
        }
    }
}

pub trait NamedType {
    fn name(&self) -> String;
}