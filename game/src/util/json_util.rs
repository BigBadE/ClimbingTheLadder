use json::JsonValue;
use crate::error;

pub fn stringify(array: &JsonValue) -> Vec<String> {
    match array {
        JsonValue::Array(array) => {
            let mut output = Vec::new();
            for value in array {
                output.push(value.dump());
            }
            return output;
        }
        _ => error!("Expected Array, found other value:\n{}", array)
    }
    return Vec::new();
}