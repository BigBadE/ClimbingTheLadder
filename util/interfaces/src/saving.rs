use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use json::JsonValue;
use json::number::Number;
use json::object::Object;

pub trait JsonSaveable {
    fn save(&self) -> JsonValue;
}

impl<T> JsonSaveable for Vec<T> where T: JsonSaveable {
    fn save(&self) -> JsonValue {
        let mut output = Vec::new();
        for value in self {
            output.push(value.save());
        }
        return JsonValue::Array(output);
    }
}

impl<T, V> JsonSaveable for HashMap<T, V> where T: ToString + Clone + Eq + Hash, V: Default + JsonSaveable {
    fn save(&self) -> JsonValue {
        let mut output = Object::new();
        for (key, value) in self {
            output.insert(key.to_string().as_str(), value.save());
        }
        return JsonValue::Object(output);
    }
}

impl JsonSaveable for String {
    fn save(&self) -> JsonValue {
        return JsonValue::String(self.clone());
    }
}

impl JsonSaveable for Duration {
    fn save(&self) -> JsonValue {
        return JsonValue::Number(Number::from(self.as_nanos() as u64));
    }
}