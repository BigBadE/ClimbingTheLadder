use crate::register::world::WorldRegister;

pub trait ModMain {
    fn handle_event(&mut self, event: ModEvent);
}

pub enum ModEvent {
    WorldRegister(WorldRegister)
}