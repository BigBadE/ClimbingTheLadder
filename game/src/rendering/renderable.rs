use crate::rendering::renderer::Renderer;

pub trait Renderable {
    fn set_handle(&mut self, renderer: &'static Box<dyn Renderer + Sync>);
}
