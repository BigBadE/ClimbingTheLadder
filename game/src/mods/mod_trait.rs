use crate::register::world::WorldAttachmentRegister;

pub trait ModMain {
    fn finish_loading(&mut self);

    fn handle_event(&mut self, event: ModEvent);
}

pub enum ModEvent {
    WorldRegister(WorldAttachmentRegister),
    KeybindRegister()
}