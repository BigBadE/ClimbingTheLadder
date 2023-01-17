use core::rendering::mesh::Mesh;
use crate::display::window::GameWindow;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        return Self {

        }
    }

    pub fn render(&mut self, window: &mut GameWindow, renderables: &[Vec<&Mesh>]) {
        for rendering in renderables {
            for mesh in rendering {
                todo!()
            }
        }
    }
}