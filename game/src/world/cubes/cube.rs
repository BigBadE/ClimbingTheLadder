use std::sync::Arc;
use crate::rendering::mesh::{FrameData, Mesh};
use crate::rendering::renderer::Renderer;
use crate::resources::resource_manager::ResourceManager;

#[derive(Copy, Clone)]
pub struct Cube {

}

impl Cube {
    pub fn new(resources: &ResourceManager, #[cfg(feature = "renderer")] renderer: &Arc<dyn Renderer>) -> Self {
        #[cfg(feature = "renderer")]
        renderer.push(Arc::new(Mesh::cube("shader".to_string())),
                      resources.asset_manager.get_texture(&"testing/dirt".to_string()).unwrap(), FrameData::new());
        return Self {
            
        }
    }

    pub fn empty() -> Self {
        return Self {
            
        }
    }
}