use std::time::Duration;
use core::settings::Settings;

pub struct GameSettings {
    pub frames_per_second: Duration,
    settings: Settings
}

impl GameSettings {
    pub fn new() -> Self {
        return Self {
            frames_per_second: Duration::from_nanos(1000000000 / 60),
            settings: Settings::new()
        }
    }

    pub fn load() -> Self {
        todo!();
    }
}