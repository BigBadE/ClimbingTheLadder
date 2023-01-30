use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use anyhow::Error;
use tokio::task::JoinSet;
use crate::mods::mod_manager::ModManager;
use crate::mods::mods::GameMod;
use crate::register::ThingRegister;
use crate::resources::content_pack::{ContentPack, load_content};
use crate::resources::resource_manager::ResourceManager;
use crate::settings::Settings;
use crate::util::alloc_handle::AllocHandle;
use crate::util::task_manager::TaskManager;
use crate::world::world::World;

pub mod language;
pub mod mods;
pub mod register;
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
    registerer: HashMap<&'static str, Box<dyn ThingRegister>>
}

impl Game {
    pub fn new(mods: JoinSet<Result<GameMod, Error>>, content: Box<dyn ContentPack>,
               mut task_manager: TaskManager) -> Self {
        let settings = Settings::new();
        let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
        load_content(&resource_manager, &mut task_manager, content);
        
        return Self {
            settings,
            task_manager,
            resource_manager,
            mods: ModManager::new(mods),
            worlds: Vec::new(),
            registerer: HashMap::new()
        };
    }

    pub async fn finish_loading(&mut self) {
        self.create_world().await;
    }

    pub async fn create_world(&mut self) -> AllocHandle {
        self.worlds.push(World::new(&self.task_manager, self.registerer.get("world").unwrap()));
        return AllocHandle::empty();
    }

    pub fn notify_update(&mut self) -> Duration {
        //Poll tasks
        let mut polled = self.task_manager.poll();
        //If one task is finished, poll the next.
        while polled.1.is_some() {
            polled.1.unwrap().call(self);
            polled = self.task_manager.poll();
        }

        //Skip update if it's running a long task.
        if polled.0 == false {
            return self.settings.updates_per_second;
        }

        for world in &mut self.worlds {
            world.update();
        }

        return self.settings.updates_per_second;
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}