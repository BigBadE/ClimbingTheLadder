use cgmath::Vector2;
use game::rendering::mesh::Mesh;

pub struct UIComponent {
    _children: Vec<UIComponent>,
    _content: Box<dyn UIContent>
}

impl UIComponent {
    pub fn new(content: Box<dyn UIContent>) -> Self {
        return Self {
            _children: Vec::new(),
            _content: content
        }
    }

    pub fn render(&self, _size: (Vector2<f32>, Vector2<f32>)) {

    }
}

pub trait UIContent {
    fn render(&self, parent: (Vector2<f32>, Vector2<f32>)) -> Vec<&Mesh>;
}