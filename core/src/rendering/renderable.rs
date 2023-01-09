use crate::rendering::mesh::Mesh;

pub trait Renderable {
    fn render<'a>(&mut self) -> &'a [Mesh];
}