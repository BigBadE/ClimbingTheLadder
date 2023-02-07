use std::path::PathBuf;
#[cfg(target_arch = "wasm32")]
use include_dir::Dir;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use game::language::language::LanguagePack;
use game::rendering::AssetType;
use crate::resources::content_pack::ContentPack;

#[derive(Clone)]
pub struct WebLoader {

}

impl ContentPack for WebLoader {
    fn types(&self) -> Vec<PathBuf> {
        todo!()
    }

    fn get_relative(&self, _subpath: &str, _path: &PathBuf) -> String {
        todo!()
    }

    fn assets(&self, _handle: &Handle, _load_first: bool) -> JoinHandle<Vec<AssetType>> {
        todo!()
    }

    fn language(&self) -> Vec<LanguagePack> {
        todo!()
    }

    fn shaders(&self, _early: bool) -> Vec<PathBuf> {
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