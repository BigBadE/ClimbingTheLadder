use std::time::Duration;
use macros::JsonResource;
use json::JsonValue;

#[derive(JsonResource)]
pub struct Settings {
    pub updates_per_second: Duration
}

impl Settings {
    pub fn new() -> Self {
        return Self {
            updates_per_second: Duration::from_nanos(1000000000 / 30)
        }
    }

    pub fn load(value: &JsonValue) -> Self {
        return __load_Settings(Settings::new(), value).unwrap();
    }
}