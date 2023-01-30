use crate::world::cubes::cube::Cube;

pub struct Block {
    cubes: [Cube; 25]
}

impl Block {
    pub fn new() -> Self {
        let mut testing = [Cube::empty(); 25];
        testing[0] = Cube::new();
        return Self {
            cubes: testing
        }
    }
}