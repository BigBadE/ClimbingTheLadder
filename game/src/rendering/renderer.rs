use std::sync::Arc;
use crate::rendering::GameTexture;
use crate::rendering::mesh::{FrameData, Mesh};

pub trait Renderer: Send + Sync {
    fn push(&self, mesh: Arc<Mesh>, texture: Arc<dyn GameTexture>, data: FrameData) -> u64;

    fn update(&self, id: u64, data: FrameData);

    fn clear(&self, id: u64);
}