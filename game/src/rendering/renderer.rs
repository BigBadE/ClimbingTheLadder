use std::collections::HashMap;
use crate::rendering::mesh::{FrameData, Mesh};

pub trait Renderer {
    fn push(&mut self, mesh: Mesh, data: FrameData) -> u64;

    fn update(&mut self, id: u64, data: FrameData);

    fn clear(&mut self, id: u64);
    
    fn get_data(&self) -> &HashMap<u64, (Mesh, FrameData)>;
}