use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use game::error;
use game::rendering::{AssetType, GameTexture};
use game::rendering::assets::AssetReference;
use game::rendering::mesh::Mesh;
use crate::resources::content_pack::ContentPack;
use game::util::alloc_handle::AllocHandle;

lazy_static! {
    pub static ref ASSET_MANAGER: Mutex<AssetManager> = Mutex::new(AssetManager::new());
}

pub struct AssetManager {
    textures: HashMap<String, Arc<dyn GameTexture>>,
    models: HashMap<String, Arc<Mesh>>
}

impl AssetManager {
    fn new() -> Self {
        return Self {
            textures: HashMap::new(),
            models: HashMap::new()
        }
    }

    pub async fn load(handle: Handle, content: Box<dyn ContentPack>, load_first: bool) -> AllocHandle {
        let assets = content.assets(&handle, load_first).await;

        match assets {
            Ok(found) => {
                let mut manager = ASSET_MANAGER.lock().unwrap();
                for asset in found {
                    match asset {
                        AssetType::Texture(textures) => for (name, tex) in textures {
                            manager.textures.insert(name, tex);
                        }
                        AssetType::Model(models) => for (name, model) in models {
                            manager.models.insert(name, model);
                        }
                    }
                }
            }
            Err(error) => error!("Internal error loading assets:\n{}", error)
        }

        return AllocHandle::empty();
    }
}

pub struct AssetReferer {}

impl AssetReference for AssetReferer {
    fn get_texture(&self, name: &String) -> Option<Arc<dyn GameTexture>> {
        return match ASSET_MANAGER.lock().unwrap().textures.get(name) {
            Some(value) => Some(value.clone()),
            None => None
        };
    }

    fn get_model(&self, name: &String) -> Option<Arc<Mesh>> {
        return match ASSET_MANAGER.lock().unwrap().models.get(name) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }
}

impl AssetReferer {
    pub fn new() -> Self {
        return Self {

        };
    }
}