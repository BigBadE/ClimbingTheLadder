use std::borrow::Borrow;
use core::rendering::mesh::Mesh;
use core::rendering::renderable::Renderable;

pub struct UIComponent {
    children: Vec<UIComponent>,
    content: Box<dyn UIContent>
}

impl UIComponent {
    pub fn new(content: Box<dyn UIContent>) -> Self {
        return Self {
            children: Vec::new(),
            content
        }
    }
}

impl Renderable for UIComponent {
    fn render(&self) -> Vec<&Mesh> {
        let mut meshes = Vec::new();
        for child in &self.children {
            for mesh in child.render() {
                meshes.push(mesh);
            }
        }
        for mesh in self.content.render() {
            meshes.push(mesh);
        }
        return meshes;
    }
}

pub trait UIContent {
    fn render(&self) -> &[Mesh];
}