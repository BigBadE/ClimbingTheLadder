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

    pub fn load(resource_loader: ResourceManager, resource: JsonValue) -> Result<Self, Error> {
        todo!()
    }
}

/*impl Renderable for UIComponent {
    fn data(&self) -> Vec<&Mesh> {
        let mut meshes = Vec::new();
        for child in &self.children {
            for mesh in child.data() {
                meshes.push(mesh);
            }
        }
        for mesh in self.content.render(Rectangle::new(
            Vector2::new(0f32, 0f32), Vector2::new(0f32, 0f32))) {
            meshes.push(mesh);
        }
        return meshes;
    }
}*/

pub trait UIContent {
    fn render(&self, parent: Rectangle) -> Vec<&Mesh>;
}