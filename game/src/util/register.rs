use crate::util::alloc_handle::AllocHandle;

pub trait ThingRegister: Send + Sync {
    fn registered(&self) -> Vec<AllocHandle>;

    fn register(&mut self, registering: fn() -> AllocHandle);

    fn finalize(&self);
}

pub struct GenericRegister {
    attachments: Vec<fn() -> AllocHandle>
}

impl ThingRegister for GenericRegister {
    fn registered(&self) -> Vec<AllocHandle> {
        let mut list = Vec::new();
        for attachment in &self.attachments {
            list.push(attachment());
        }
        return list;
    }

    fn register(&mut self, registering: fn() -> AllocHandle) {
        self.attachments.push(registering);
    }

    fn finalize(&self) {
        todo!()
    }
}

impl GenericRegister {
    pub fn new() -> Self {
        return GenericRegister {
            attachments: Vec::new(),
        };
    }

    pub fn from(attachments: Vec<fn() -> AllocHandle>) -> Self {
        return GenericRegister {
            attachments
        };
    }
}