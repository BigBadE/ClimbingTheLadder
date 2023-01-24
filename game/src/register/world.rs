use crate::register::ThingRegister;
use crate::util::alloc_handle::AllocHandle;

pub struct WorldRegister {
    attachments: Vec<fn() -> AllocHandle>
}

impl ThingRegister for WorldRegister {
    fn registered(&self) -> Vec<AllocHandle> {
        let mut list = Vec::new();
        for attachment in &self.attachments {
            list.push((attachment)());
        }
        return list;
    }

    fn register(&mut self, registering: fn() -> AllocHandle) {
        self.attachments.push(registering);
    }
}

impl WorldRegister {

}