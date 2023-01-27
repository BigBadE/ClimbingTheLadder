use crate::rendering::mesh::{FrameData, Mesh};

pub trait Renderer {
    fn push(&self, mesh: Mesh, data: FrameData) -> u64;

    fn update(&self, id: u64, data: FrameData);

    fn clear(&self, id: u64);
}