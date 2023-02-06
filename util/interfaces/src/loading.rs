use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use anyhow::Error;
use json::JsonValue;

pub trait JsonLoadable: Sized {
    fn load(value: &JsonValue) -> Result<Self, Error>;
}

impl<T> JsonLoadable for Vec<T> where T: JsonLoadable {
    fn load(value: &JsonValue) -> Result<Vec<T>, Error> {
        let mut result = Vec::new();
        match value {
            JsonValue::Array(values) => {
                for value in values {
                    result.push(T::load(&value)?);
                }
            }
            _ => return Err(Error::msg(format!("Non-array type in array! {:?}", value)))
        }
        return Ok(result);
    }
}

impl<T, V> JsonLoadable for HashMap<T, V> where T: From<String> + Eq + Hash, V: Default + JsonLoadable {
    fn load(value: &JsonValue) -> Result<HashMap<T, V>, Error> {
        let mut output = HashMap::new();
        for (key, value) in value.entries() {
            output.insert(T::from(key.to_string()), V::load(value)?);
        }
        return Ok(output);
    }
}

impl JsonLoadable for String {
    fn load(value: &JsonValue) -> Result<String, Error> {
        return match value {
            JsonValue::String(found) => Ok(found.clone()),
            _ => Err(Error::msg(format!("Expected string, found {:?}", value)))
        }
    }
}

impl JsonLoadable for Duration {
    fn load(value: &JsonValue) -> Result<Self, Error> {
        return match value {
            JsonValue::Number(value) => match value.as_fixed_point_u64(0) {
                Some(number) => Ok(Duration::from_nanos(number)),
                None => Err(Error::msg(format!("Expected positive number, got negative/NaN {:?}", value)))
            }
            _ => Err(Error::msg(format!("Expected number, found {:?}", value)))
        }
    }
}