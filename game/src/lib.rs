use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use anyhow::Error;
use tokio::task::JoinSet;
use crate::mods::mod_manager::ModManager;
use crate::mods::mods::GameMod;
use crate::register::ThingRegister;
use crate::resources::content_pack::ContentPack;
use crate::resources::resource_manager::ResourceManager;
use crate::settings::Settings;
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
    pub resource_manager: ResourceManager,
    worlds: Vec<World>,
    mods: ModManager,
    registerer: HashMap<&'static str, Box<dyn ThingRegister + Send + Sync>>,
}

impl Game {
    pub fn new(mods: JoinSet<Result<GameMod, Error>>, content: Box<dyn ContentPack + Send>,
                     task_manager: TaskManager) -> Self {
        let settings = Settings::new();
        let resource_manager = ResourceManager::new();

        task_manager.queue(true, ResourceManager::l)
        return Self {
            settings,
            task_manager,
            resource_manager,
            mods: ModManager::new(mods),
            worlds: Vec::new(),
            registerer: HashMap::new()
        };
    }

    pub fn finish_loading(&mut self) {

    }

    pub async fn create_world(&mut self) {
        self.worlds.push(World::new(&self.task_manager, self.registerer.get("world").unwrap()));
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