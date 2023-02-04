use cgmath::Vector2;

pub mod alloc_handle;
pub mod json_util;
pub mod macros;
pub mod runtime_factory;
pub mod task_manager;

pub struct Rectangle {
    pub size: Vector2<u32>,
    pub position: Vector2<u32>
}

impl Rectangle {
    pub fn new(size: Vector2<u32>, position: Vector2<u32>) -> Self {
        return Self {
            size,
            position
        }
    }
}
