use std::sync::Arc;
use game::rendering::renderer::Renderer;
use crate::ui::window::UIWindow;

pub struct UIManager {
    pub cursor_pos: (f64, f64),
    pub size: (u32, u32),
    pub windows: Vec<UIWindow>,
    pub renderer: &'static Arc<dyn Renderer>,
}

impl UIManager {
    pub fn new(renderer: &'static Arc<dyn Renderer>) -> Self {
        return Self {
            cursor_pos: (0f64, 0f64),
            size: (0, 0),
            windows: Vec::new(),
            renderer
        }
    }

    pub fn update(&mut self) {
        for window in &mut self.windows {
            window.update();
        }
    }

    pub fn loading(&mut self) {

    }

    pub fn open(&mut self, window: UIWindow) {
        self.windows.push(window);
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;
    }
}