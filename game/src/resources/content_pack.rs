use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::language::language::{LANGUAGE_MANAGER, LanguagePack};
use crate::resources::resource_manager::ResourceManager;
use crate::util::alloc_handle::AllocHandle;
use crate::util::task_manager::TaskManager;

pub trait ContentPack: Send {
    fn shaders(&self) -> Vec<(String, String)>;

    //This is ignored for mods because they're not loaded yet. Only used for UI shaders.
    fn load_first_shaders(&self) -> Vec<(String, String)>;

    fn types(&self) -> Vec<PathBuf>;

    fn language(&self) -> Vec<LanguagePack>;
}

pub fn load_content(resource_manager: &Arc<Mutex<ResourceManager>>,
                          task_manager: &mut TaskManager, content: Box<dyn ContentPack>) {
    ResourceManager::load_all(resource_manager, task_manager, &content);
    task_manager.queue(true, get_packs(content));
}

async fn get_packs(content: Box<dyn ContentPack>) -> AllocHandle {
    LANGUAGE_MANAGER.write().unwrap().load_packs(content.language());
    return AllocHandle::empty();
}
