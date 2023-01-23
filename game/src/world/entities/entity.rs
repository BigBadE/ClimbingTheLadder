use std::sync::atomic::AtomicU64;
use crate::world::world::WorldData;

static mut NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub struct Entity {
    id: u64
}

impl Entity {
    //Creates a new entity. These aren't in a world yet, and must be spawned
    pub fn new() -> Self {
        return Self {
            id: Self::get_next_id()
        }
    }

    pub fn spawn(mut self, world: &mut WorldData) {

    }

    fn get_next_id() -> u64 {
        //Unsafe due to race conditions, but atomic u64 fixes that
        let id = unsafe { *NEXT_ID.get_mut() };
        unsafe { NEXT_ID = AtomicU64::new(id + 1) }
        return id;
    }
}