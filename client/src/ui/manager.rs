use winit::event::{ElementState, KeyboardInput, MouseButton};
use core::rendering::renderable::Renderable;
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

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;
    }
}

impl Renderable for UIManager {
    fn render<'a>(&self) -> &'a [&core::rendering::mesh::Mesh] {
        let mut meshes = Vec::new();
        for window in self.windows {
            for mesh in window.render() {
                meshes.push(*mesh);
            }
        }
        return meshes.as_slice();
    }
}