use crate::util::alloc_handle::AllocHandle;

pub mod world;

pub trait ThingRegister: Send + Sync {
    fn registered(&self) -> Vec<AllocHandle>;

    //Due to Rust shenanigans, the type here can't be verified at compile time.
    //The game will check types at runtime, make sure to see what type each register wants.
    fn register(&mut self, registering: fn() -> AllocHandle);
}