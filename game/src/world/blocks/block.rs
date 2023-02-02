use std::sync::Arc;
use crate::rendering::renderer::Renderer;
use crate::resources::resource_manager::ResourceManager;
use crate::world::cubes::cube::Cube;

pub struct Block {
    cubes: [Cube; 25]
}

impl Block {
    pub fn new(resources: &ResourceManager, #[cfg(feature = "renderer")] arc: Arc<dyn Renderer>) -> Self {
        let mut testing = [Cube::empty(); 25];
        testing[0] = Cube::new(resources, #[cfg(feature = "renderer")] &arc);
        return Self {
            cubes: testing
        }
    }
}