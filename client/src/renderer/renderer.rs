use game::rendering::renderer::Renderer;
use crate::display::window::GameWindow;

pub struct GameRenderer {}

impl GameRenderer {
    pub fn new() -> Self {
        return Self {

        }
    }

    pub fn render(&mut self, window: &mut GameWindow) {
        todo!()
    }
}

impl Renderer for GameRenderer {
    
}