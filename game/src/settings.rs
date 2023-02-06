use std::time::Duration;
use macros::JsonLoadable;

#[derive(JsonLoadable)]
pub struct Settings {
    pub updates_per_second: Duration
}

impl Default for Settings {
    fn default() -> Self {
        return Self {
            updates_per_second: Duration::from_nanos(1000000000 / 30)
        }
    }
}