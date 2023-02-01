use std::sync::Arc;
use crate::rendering::renderer::Renderer;
use crate::world::blocks::block::Block;

pub struct Room {
    blocks: Vec<Block>,
}

impl Room {
    pub fn new(#[cfg(feature = "renderer")] arc: Arc<dyn Renderer>) -> Self {
        #[cfg(feature = "renderer")]
            let blocks = vec!(Block::new(arc));
        #[cfg(not(feature = "renderer"))]
            let blocks = vec!(Block::new());

        return Self {
            blocks
        };
    }

    pub fn update(&mut self) {}
}