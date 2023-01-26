use std::sync::{Arc, Mutex};
use crate::rendering::renderer::Renderer;

pub trait Renderable {
    fn set_handle(&mut self, renderer: &mut Arc<Mutex<Box<dyn Renderer>>>);
}
