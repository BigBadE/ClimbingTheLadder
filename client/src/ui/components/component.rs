use std::borrow::Borrow;
use anyhow::Error;
use json::JsonValue;
use core::rendering::mesh::Mesh;
use core::rendering::renderable::Renderable;
use macros::JsonResource;
use core::resources::resource_loader::{ResourceImpl, ResourceLoader};

#[derive(JsonResource)]
pub struct UIComponent {
    #[ignore_field]
    children: Vec<UIComponent>,
    #[ignore_field]
    content: Box<dyn UIContent>
}

impl UIComponent {
    pub fn new(content: Box<dyn UIContent>) -> Self {
        return Self {
            children: Vec::new(),
            content
        }
    }

    pub fn load(resource_loader: ResourceLoader, resource: JsonValue) -> Result<Self, Error> {
        let loading = resource_loader.get_implementor(resource["content"])?;

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

pub trait SavableUI: UIContent + ResourceImpl {

}

pub trait UIContent {
    fn render(&self) -> &[Mesh];
}