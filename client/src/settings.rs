use std::time::Duration;
use core::settings::Settings;
use macros::JsonResource;
use json::JsonValue;
use crate::input::manager::InputManager;

#[derive(JsonResource)]
pub struct GameSettings {
    pub frames_per_second: Duration,
    pub inputs: InputManager,
    pub settings: Settings
}

impl GameSettings {
    pub fn new() -> Self {
        return Self {
            frames_per_second: Duration::from_nanos(1000000000 / 60),
            inputs: InputManager::new(),
            settings: Settings::new()
        };
    }

    pub fn load(resource: &JsonValue) -> Self {
        return __load_GameSettings(GameSettings::new(), resource);
    }
}