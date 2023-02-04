use std::sync::Arc;
use crate::rendering::renderer::Renderer;
use crate::resources::resource_manager::ResourceManager;
use crate::world::blocks::block::Block;

pub struct Room {
    _blocks: Vec<Block>,
}

impl Room {
    pub fn new(resources: &ResourceManager, #[cfg(feature = "renderer")] arc: Arc<dyn Renderer>) -> Self {
        #[cfg(feature = "renderer")]
            let blocks = vec!(Block::new(resources, arc));
        #[cfg(not(feature = "renderer"))]
            let blocks = vec!(Block::new());

        return Self {
            _blocks: blocks
        };
    }

    pub fn update(&mut self) {}
}