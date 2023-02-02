use std::sync::Arc;
use crate::rendering::GameTexture;
use crate::rendering::mesh::Mesh;
use crate::resources::content_pack::ContentPack;
use crate::util::task_manager::TaskManager;

pub trait AssetReference: Send {
    fn get_texture(&self, name: &String) -> Option<Arc<dyn GameTexture>>;

    fn get_model(&self, name: &String) -> Option<Arc<Mesh>>;

    fn load(&self, task_manager: &mut TaskManager, content: Box<dyn ContentPack>);
}