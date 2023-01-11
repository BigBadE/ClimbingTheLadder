use crate::rendering::mesh::Mesh;

pub trait Renderable {
    fn render<'a>(&self) -> &'a [&Mesh];
}