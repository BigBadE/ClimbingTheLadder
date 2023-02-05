use std::sync::Arc;
use crate::rendering::mesh::{FrameData, Mesh};
use crate::resources::resource_manager::ResourceManager;

#[derive(Copy, Clone)]
pub struct Cube {

}

impl Cube {
    pub fn new(resources: &ResourceManager) -> Self {
        #[cfg(feature = "renderer")]
        resources.renderer.push(Arc::new(Mesh::cube("shader".to_string())),
                      resources.asset_manager.get_texture(&"testing/dirt".to_string()).unwrap(), FrameData::new());
        return Self {
            
        }
    }

    pub fn empty() -> Self {
        return Self {
            
        }
    }
}