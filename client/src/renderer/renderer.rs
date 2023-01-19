use core::rendering::mesh::Mesh;
use game::Game;
use game::rendering::renderer::Renderer;
use crate::display::window::GameWindow;

pub struct GameRenderer {}

impl GameRenderer {
    pub fn new() -> Self {
        return Self {

        }
    }

    pub fn render(&mut self, window: &mut GameWindow, renderables: Vec<&Mesh>) {
        for rendering in renderables {
            for mesh in rendering {
                todo!()
            }
        }
    }
}

impl Renderer for GameRenderer {
    
}