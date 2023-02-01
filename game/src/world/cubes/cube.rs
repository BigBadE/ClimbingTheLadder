use std::sync::Arc;
use crate::rendering::mesh::{FrameData, Mesh};
use crate::rendering::renderer::Renderer;
use crate::rendering::texture::ColorTexture;

#[derive(Copy, Clone)]
pub struct Cube {

}

impl Cube {
    pub fn new(#[cfg(feature = "renderer")] arc: &Arc<dyn Renderer>) -> Self {
        #[cfg(feature = "renderer")]
        arc.push(Arc::new(Mesh::cube("shader".to_string())),
                 Arc::new(ColorTexture::new([255, 0, 0, 255])), FrameData::new());
        return Self {
            
        }
    }

    pub fn empty() -> Self {
        return Self {
            
        }
    }
}