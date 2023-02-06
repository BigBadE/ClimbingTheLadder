use std::time::Duration;
use macros::{JsonLoadable, JsonSaveable};
use crate::input::manager::InputManager;

#[derive(JsonLoadable, JsonSaveable)]
pub struct GameSettings {
    pub frames_per_second: Duration,
    pub inputs: InputManager
}

impl Default for GameSettings {
    fn default() -> Self {
        return Self {
            frames_per_second: Duration::from_nanos(1000000000 / 60),
            inputs: InputManager::default()
        };
    }
}