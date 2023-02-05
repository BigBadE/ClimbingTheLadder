use std::path::PathBuf;
use tokio::runtime::Handle;
#[cfg(target_arch = "wasm32")]
use include_dir::Dir;
use tokio::task::JoinHandle;
use game::language::language::LanguagePack;
use game::rendering::AssetType;
use game::resources::content_pack::ContentPack;
use game::util::task_manager::TaskManager;

#[derive(Clone)]
pub struct WebLoader {

}

impl ContentPack for WebLoader {
    fn load(&self, task_manager: &mut TaskManager) {
        todo!()
    }

    fn early_load(&self, task_manager: &mut TaskManager) {
        todo!()
    }

    fn types(&self) -> Vec<PathBuf> {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn ContentPack> {
        todo!()
    }

    fn get_relative(&self, _path: PathBuf) -> String {
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