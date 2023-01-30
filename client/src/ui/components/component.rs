use anyhow::Error;
use json::JsonValue;
use game::rendering::mesh::Mesh;
use game::resources::resource_manager::{ResourceManager};
use game::util::types::Rectangle;

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

    pub fn render(&self, size: Rectangle) {

    }
}

pub trait UIContent {
    fn render(&self, parent: Rectangle) -> Vec<&Mesh>;
}