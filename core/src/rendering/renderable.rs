use crate::rendering::mesh::Mesh;

pub trait Renderable {
    fn render(&self) -> Vec<&Mesh>;
}