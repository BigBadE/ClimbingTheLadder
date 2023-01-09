use core::time::Duration;

pub struct Settings {
    pub updates_per_second: Duration
}

impl Settings {
    pub fn new() -> Self {
        return Self {
            updates_per_second: Duration::from_nanos(1000000000 / 30)
        }
    }

    pub fn load() -> Self {
        todo!();
    }
}