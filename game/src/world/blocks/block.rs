use crate::resources::resource_manager::ResourceManager;
use crate::world::cubes::cube::Cube;

pub struct Block {
    _cubes: [Cube; 25]
}

impl Block {
    pub fn new(resources: &ResourceManager) -> Self {
        let mut testing = [Cube::empty(); 25];
        testing[0] = Cube::new(resources);
        return Self {
            _cubes: testing
        }
    }
}