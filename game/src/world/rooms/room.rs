use std::sync::Arc;
use crate::rendering::renderer::Renderer;
use crate::resources::resource_manager::ResourceManager;
use crate::world::blocks::block::Block;

pub struct Room {
    _blocks: Vec<Block>,
}

impl Room {
    pub fn new(resources: &ResourceManager) -> Self {
        let blocks = vec!(Block::new(resources));

        return Self {
            _blocks: blocks
        };
    }

    pub fn update(&mut self) {}
}