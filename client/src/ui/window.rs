use core::rendering::renderable::Renderable;
use crate::ui::components::component::UIComponent;
use core::rendering::mesh::Mesh;

pub struct UIWindow {
    content: UIComponent
}

impl UIWindow {
    pub fn new(content: UIComponent) -> Self {
        return Self {
            content
        }
    }
}

impl Renderable for UIWindow {
    fn render(&self) -> Vec<&Mesh> {
        return self.content.render();
    }
}