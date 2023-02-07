use std::fs;
use std::future::Future;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use anyhow::Error;
use json::JsonValue;
use json::object::Object;
use tokio::runtime::Handle;
use tokio::task::{JoinError, JoinSet};
use game::{error, Game, LoadingStage};
use game::language::language::LANGUAGE_MANAGER;
use crate::resources::content_pack::ContentPack;
use game::resources::resource_loader::ResourceLoader;
use game::resources::resource_manager::ResourceManager;
use game::util::alloc_handle::AllocHandle;
use game::util::task_manager::TaskManager;
use crate::display::window::GameWindow;
use crate::renderer::assets::AssetManager;
use crate::renderer::shaders::ShaderManager;

pub fn load(window: &GameWindow, content: &Box<dyn ContentPack>, resource_manager: &Arc<Mutex<ResourceManager>>, task_manager: &mut TaskManager) {
    let resource_loader = Arc::new(Mutex::new(ResourceLoader::new(resource_manager.clone())));

    for json in content.types() {
        let loader = task_manager.get_runtime(true).spawn(load_json(json.clone()));
        task_manager.queue(false,
                           load_types(loader, content.get_relative("", &json), resource_loader.clone(),
                                               task_manager.get_runtime(false).clone()));
    }

    let runtime = task_manager.get_runtime(true).clone();
    task_manager.queue(false, load_language(runtime.clone(), content.clone_boxed()));
    task_manager.queue(false, ShaderManager::load(false, window.device.clone(), window.config.clone(),
                                                  ShaderManager::get_shaders(false, runtime.clone(), content.clone_boxed())));
    task_manager.queue(false, AssetManager::load(runtime.clone(), content.clone_boxed(), false));
}

pub fn finish_load(game: &mut Game, _: AllocHandle) {
    game.loaded = LoadingStage::Finished;
    game.registerer.get("keyaction").unwrap();
    game.task_manager.queue_after(false, Game::finish_loading(
        game.task_manager.get_runtime(false).clone(), game.resource_manager.clone(),
        AllocHandle::convert(game.registerer.get("world").unwrap().registered())), Game::done_loading);
}

pub fn early_load(window: &GameWindow, content: &Box<dyn ContentPack>, task_manager: &mut TaskManager) {
    let runtime = task_manager.get_runtime(true).clone();

    task_manager.queue(false, ShaderManager::load(true, window.device.clone(), window.config.clone(),
                                                  ShaderManager::get_shaders(true, runtime.clone(), content.clone_boxed())));
    task_manager.queue(false, AssetManager::load(runtime.clone(), content.clone_boxed(), true));
}

pub async fn load_language(_handle: Handle, content: Box<dyn ContentPack>) -> AllocHandle {
    //TODO speed these up
    LANGUAGE_MANAGER.write().unwrap().load_packs(content.language());

    return AllocHandle::empty();
}

pub async fn load_json(path: PathBuf) -> Result<JsonValue, Error> {
    return Ok(json::parse(String::from_utf8(fs::read(path)?)?.as_str())?);
}

pub async fn load_types(loading: impl Future<Output=Result<Result<JsonValue, Error>, JoinError>>,
                    name: String, loader: Arc<Mutex<ResourceLoader>>, runtime: Handle) -> AllocHandle {
    let found = match loading.await {
        Ok(value) => match value {
            Ok(value) => value,
            Err(error) => {
                error!("Error loading JSON {}: {}", name, error);
                return AllocHandle::empty();
            }
        },
        Err(error) => {
            error!("Error joining thread: {}", error);
            return AllocHandle::empty();
        }
    };

    let mut join_set = JoinSet::new();
    for found in get_types(found) {
        join_set.spawn_on(ResourceLoader::spawn(loader.clone(), found), &runtime);
    }

    while let Some(value) = join_set.join_next().await {
        match value {
            Ok(result) => match result {
                Ok(_) => {},
                Err(error) => error!("Error loading JSON resource {}:\n{}", name, error)
            }
            Err(error) => error!("Error joining resource loading thread:\n{}", error)
        }
    }

    return AllocHandle::empty();
}

pub fn get_types(found: JsonValue) -> Vec<Object> {
    let mut types = Vec::new();
    match found {
        JsonValue::Array(values) => {
            for value in values {
                match value {
                    JsonValue::Object(object) => types.push(object),
                    _ => {
                        error!("Tried to load JSON with unknown top level object: {}", value);
                        continue;
                    }
                }
            }
        }
        JsonValue::Object(object) => types.push(object),
        _ => {
            error!("Tried to load JSON with unknown top level object: {}", found);
            return Vec::new();
        }
    }
    return types;
}