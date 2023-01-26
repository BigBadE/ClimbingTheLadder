use std::sync::{Arc, Mutex};
use game::rendering::mesh::{FrameData, Mesh, Vertex};
use game::rendering::renderable::Renderable;
use game::rendering::renderer::Renderer;
use crate::ui::window::UIWindow;

pub struct UIManager {
    pub cursor_pos: (f64, f64),
    pub size: (u32, u32),
    pub windows: Vec<UIWindow>
}

impl UIManager {
    pub fn new() -> Self {
        return Self {
            cursor_pos: (0f64, 0f64),
            size: (0, 0),
            windows: Vec::new()
        }
    }

    pub fn update() {

    }

    pub fn open(&mut self, window: UIWindow) {
        self.windows.push(window);
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;
    }
}

impl Renderable for UIManager {
    fn set_handle(&mut self, renderer: &mut Arc<Mutex<Box<dyn Renderer>>>) {
        let mut mesh = Mesh::new("shader".to_string());
        mesh.vertexes.push(Vertex::new([0f32, 0f32, 0f32], [0f32, 0f32]));
        mesh.vertexes.push(Vertex::new([10f32, 0f32, 0f32], [0f32, 0f32]));
        mesh.vertexes.push(Vertex::new([10f32, 10f32, 0f32], [0f32, 0f32]));
        mesh.indices.push(0);
        mesh.indices.push(1);
        mesh.indices.push(2);
        renderer.lock().unwrap().push(mesh, FrameData::new());
    }
}