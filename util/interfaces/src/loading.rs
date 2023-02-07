use std::collections::HashMap;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::ptr;
use std::time::Duration;
use anyhow::Error;
use json::JsonValue;
use json::number::Number;

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

impl<T, const LEN: usize> JsonLoadable for [T; LEN] where T: JsonLoadable + Default + Sized {
    fn load(value: &JsonValue) -> Result<Self, Error> {
        let mut array: [MaybeUninit<T>; LEN] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut i = 0;
        match value {
            JsonValue::Array(values) => {
                if i != values.len() {
                    return Err(Error::msg(format!("Bad length in array: {:?}", value)))
                }
                for value in values {
                    array[i] = MaybeUninit::new(T::load(&value)?);
                    i += 1;
                }
            }
            _ => return Err(Error::msg(format!("Non-array type in array! {:?}", value)))
        }
        //Do I know if this works? No. Will this break later? Yes. Does it make the red line go away? For now.
        return Ok(unsafe {ptr::read(&array as *const [MaybeUninit<T>; LEN] as *const [T; LEN]) });
    }
}

//This is required or else TryFrom<T> could conflict with other impls.
pub trait LoadableNumber: TryFrom<Number> + Into<Number> + Copy {}

impl<T> JsonLoadable for T where T: LoadableNumber {
    fn load(value: &JsonValue) -> Result<Self, Error> {
        return match value {
            JsonValue::Number(number) => match TryFrom::try_from(*number) {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::msg(format!("Expected number, found {:?}", value)))
            },
            _ => Err(Error::msg(format!("Expected number, found {:?}", value)))
        }
    }
}

impl LoadableNumber for u8 {}
impl LoadableNumber for u16 {}
impl LoadableNumber for u32 {}
impl LoadableNumber for u64 {}
impl LoadableNumber for i8 {}
impl LoadableNumber for i16 {}
impl LoadableNumber for i32 {}
impl LoadableNumber for i64 {}
impl LoadableNumber for f32 {}
impl LoadableNumber for f64 {}
