#![feature(stmt_expr_attributes)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Handle;
use crate::mods::mod_manager::ModManager;
use crate::mods::ModProvider;
use crate::mods::mods::GameMod;
use crate::register::ThingRegister;
use crate::register::world::WorldAttachmentRegister;
use crate::rendering::assets::AssetReference;
use crate::rendering::renderer::Renderer;
use crate::resources::content_pack::{ContentPack, load_content};
use crate::resources::resource_manager::ResourceManager;
use crate::settings::Settings;
use crate::util::alloc_handle::AllocHandle;
use crate::util::task_manager::TaskManager;
use crate::world::world::World;

pub mod language;
pub mod mods;
pub mod register;
#[cfg(feature = "renderer")]
pub mod rendering;
pub mod resources;
pub mod util;
pub mod world;
pub mod settings;

pub struct Game {
    pub settings: Settings,
    pub task_manager: TaskManager,
    pub resource_manager: Arc<Mutex<ResourceManager>>,
    worlds: Vec<World>,
    mods: ModManager,
    registerer: HashMap<&'static str, Arc<dyn ThingRegister>>,
    loaded: bool,
    #[cfg(feature = "renderer")]
    renderer: Arc<dyn Renderer>
}

impl Game {
    pub fn new(mods: Box<dyn ModProvider>, content: Box<dyn ContentPack>, mut task_manager: TaskManager,
               #[cfg(feature = "renderer")]asset_reference: Box<dyn AssetReference>,
               #[cfg(feature = "renderer")]renderer: Arc<dyn Renderer>) -> Self {
        println!("Starting game");
        let settings = Settings::new();
        #[cfg(feature = "renderer")]
        let resource_manager = Arc::new(Mutex::new(ResourceManager::new(asset_reference)));
        #[cfg(not(feature = "renderer"))]
            let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
        load_content(&resource_manager, &mut task_manager, content);
        let mods = ModManager::new(mods.get_mods(&task_manager.get_runtime(false)));

        return Self {
            settings,
            task_manager,
            resource_manager,
            mods,
            worlds: Vec::new(),
            registerer: hashmap!("world" => WorldAttachmentRegister::get_registerer()),
            loaded: false,
            #[cfg(feature = "renderer")]
            renderer
        };
    }

    pub async fn finish_loading(handle: Handle, #[cfg(feature = "renderer")] renderer: Arc<dyn Renderer>,
                                resources: Arc<Mutex<ResourceManager>>, found_attachments: Arc<dyn ThingRegister>) -> AllocHandle {
        resources.lock().unwrap().asset_manager.get_texture(&"testing/dirt".to_string()).unwrap();
        #[cfg(feature = "renderer")]
        return Self::create_world(handle, renderer, resources, found_attachments).await;
        #[cfg(not(feature = "renderer"))]
        return Self::create_world(handle, resources, found_attachments).await;
    }

    pub async fn create_world(handle: Handle, #[cfg(feature = "renderer")] renderer: Arc<dyn Renderer>,
                              resources: Arc<Mutex<ResourceManager>>, found_attachments: Arc<dyn ThingRegister>) -> AllocHandle {
        #[cfg(feature = "renderer")]
        return AllocHandle::new(World::new(&handle, renderer,
                                           resources, found_attachments));
        #[cfg(not(feature = "renderer"))]
        return AllocHandle::new(World::new(&handle, resources, found_attachments));
    }

    pub async fn notify_update(&mut self) -> Duration {
        //Poll tasks
        let mut polled = self.task_manager.poll().await;
        //If one task is finished, poll the next.
        while polled.1.is_some() {
            polled.1.unwrap().call(self);
            polled = self.task_manager.poll().await;
        }

        //Skip update if it's running a long task.
        if polled.0 == false {
            return self.settings.updates_per_second;
        }

        if !self.loaded {
            self.loaded = true;
            let loader = Self::finish_loading(self.task_manager.get_runtime(false).clone(),
                                              #[cfg(feature = "renderer")] self.renderer.clone(),
                                              self.resource_manager.clone(), self.registerer.get("world").unwrap().clone());

            self.task_manager.queue_after(false, loader, Self::done_loading);
            return self.settings.updates_per_second;
        }

        let mut removed = usize::MAX;
        let mut i = 0;
        for world in &mut self.worlds {
            if world.update().is_err() {
                removed = i;
            }
            i += 1;
        }

        if removed != usize::MAX {
            self.worlds.remove(removed);
        }

        return self.settings.updates_per_second;
    }

    fn done_loading(game: &mut Game, world: AllocHandle) {
        game.worlds.push(world.deref());
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}