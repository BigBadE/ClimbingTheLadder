use winit::event::{ElementState, KeyboardInput, MouseButton};
use core::rendering::renderable::Renderable;

pub struct UIManager {
    pub cursor_pos: (f64, f64),
    pub size: (u32, u32)
}

impl UIManager {
    pub fn new() -> Self {
        return Self {
            cursor_pos: (0f64, 0f64),
            size: (0, 0)
        }
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    fn key_input(&mut self, input: KeyboardInput) {
        todo!()
    }

    fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        todo!()
    }
}

impl Renderable for UIManager {
    fn render<'a>(&mut self) -> &'a [core::rendering::mesh::Mesh] {
        todo!()
    }
}