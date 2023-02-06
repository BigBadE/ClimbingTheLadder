use std::sync::Arc;
use crate::rendering::GameTexture;
use crate::rendering::mesh::Mesh;

pub trait AssetReference: Send {
    fn get_texture(&self, name: &String) -> Option<Arc<dyn GameTexture>>;

    fn get_model(&self, name: &String) -> Option<Arc<Mesh>>;
}