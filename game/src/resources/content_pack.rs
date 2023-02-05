use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::resources::resource_manager::ResourceManager;
use crate::util::task_manager::TaskManager;

pub trait ContentPack: Send + Sync {
    fn load(&self, resource_manager: &Arc<Mutex<ResourceManager>>, task_manager: &mut TaskManager);

    fn early_load(&self, task_manager: &mut TaskManager);

    fn types(&self) -> Vec<PathBuf>;

    fn clone_boxed(&self) -> Box<dyn ContentPack>;

    fn get_relative(&self, path: PathBuf) -> String;
}