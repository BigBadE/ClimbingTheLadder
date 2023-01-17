use crate::rendering::mesh::{FrameData, Mesh};

pub trait Renderable {
    fn data(&self) -> Vec<&Mesh>;

    fn render(&self) -> FrameData;
}