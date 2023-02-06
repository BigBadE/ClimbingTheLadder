use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use game::language::language::LanguagePack;
use game::rendering::AssetType;

pub trait ContentPack: Send + Sync {
    fn types(&self) -> Vec<PathBuf>;

    fn get_relative(&self, subpath: &str, path: &PathBuf) -> String;

    fn assets(&self, handle: &Handle, load_first: bool) -> JoinHandle<Vec<AssetType>>;

    fn language(&self) -> Vec<LanguagePack>;

    fn shaders(&self, early: bool) -> Vec<PathBuf>;

    fn clone_boxed(&self) -> Box<dyn ContentPack>;
}