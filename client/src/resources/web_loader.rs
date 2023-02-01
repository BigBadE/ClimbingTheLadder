use std::path::PathBuf;
use tokio::runtime::Handle;
#[cfg(target_arch = "wasm32")]
use include_dir::Dir;
use tokio::task::JoinHandle;
use game::language::language::LanguagePack;
use game::rendering::AssetType;
use game::resources::content_pack::ContentPack;

#[derive(Clone)]
pub struct WebLoader {

}

impl ContentPack for WebLoader {
    fn shaders(&self) -> Vec<(String, String)> {
        todo!()
    }

    fn load_first_shaders(&self) -> Vec<(String, String)> {
        todo!()
    }

    fn types(&self) -> Vec<PathBuf> {
        todo!()
    }

    fn assets(&self, handle: &Handle) -> JoinHandle<Vec<AssetType>> {
        todo!()
    }
    
    fn language(&self) -> Vec<LanguagePack> {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn ContentPack> {
        todo!()
    }
}

impl WebLoader {
    #[cfg(target_arch = "wasm32")]
    pub fn new(resources: Dir<'_>) -> Self {
        return Self {

        }
    }
}