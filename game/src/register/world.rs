use crate::register::ThingRegister;
use crate::util::alloc_handle::AllocHandle;

pub struct WorldAttachmentRegister {
    attachments: Vec<fn() -> AllocHandle>
}

impl ThingRegister for WorldAttachmentRegister {
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

impl WorldAttachmentRegister {
    pub fn get_registerer() -> Box<dyn ThingRegister> {
        let found = WorldAttachmentRegister {
            attachments: Vec::new()
        };

        return Box::new(found);
    }
}