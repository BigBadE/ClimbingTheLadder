use crate::world::blocks::block::Block;

pub struct Room {
    blocks: Vec<Block>
}

impl Room {
    pub fn new() -> Self {
        return Self {
            blocks: vec!(Block::new())
        }
    }
    
    pub fn update(&mut self) {
        
    }
}