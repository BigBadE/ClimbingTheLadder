pub mod assets;
pub mod mesh;
pub mod renderer;
pub mod texture;

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use crate::rendering::mesh::Mesh;

#[cfg(feature = "renderer")]
pub enum AssetType {
    Model(HashMap<String, Arc<Mesh>>),
    Texture(HashMap<String, Arc<dyn GameTexture>>)
}

pub trait GameTexture: Send + Sync + Display {
    fn dimensions(&self) -> (u32, u32);

    fn name(&self) -> String;

    fn data(&self) -> &[u8];
}