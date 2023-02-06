#![feature(stmt_expr_attributes)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Handle;
use crate::mods::mods::GameMod;
use crate::resources::resource_manager::ResourceManager;
use crate::settings::Settings;
use crate::util::alloc_handle::AllocHandle;
use crate::util::register::{GenericRegister, ThingRegister};
use crate::util::task_manager::TaskManager;
use crate::world::attachments::WorldAttachment;
use crate::world::world::World;

pub mod language;
pub mod mods;
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
    pub loaded: LoadingStage,
    pub registerer: HashMap<&'static str, Box<dyn ThingRegister>>,
    worlds: Vec<World>,
}

impl Game {
    pub fn new(resource_manager: Arc<Mutex<ResourceManager>>, task_manager: TaskManager,
               mut registerer: HashMap<&'static str, Box<dyn ThingRegister>>) -> Self {
        println!("Starting game");
        let settings = Settings::default();

        Self::add_registers(&mut registerer);

        return Self {
            settings,
            task_manager,
            resource_manager,
            worlds: Vec::new(),
            registerer,
            loaded: LoadingStage::Early
        };
    }

    fn add_registers(registerer: &mut HashMap<&'static str, Box<dyn ThingRegister>>) {
        registerer.insert("world", Box::new(GenericRegister::from(vec!())));
    }

    pub async fn finish_loading(handle: Handle, resources: Arc<Mutex<ResourceManager>>,
    found_attachments: Vec<Box<dyn WorldAttachment>>) -> AllocHandle {
        return Self::create_world(handle, resources, found_attachments).await;
    }

    pub async fn create_world(handle: Handle, resources: Arc<Mutex<ResourceManager>>,
                              found_attachments: Vec<Box<dyn WorldAttachment>>) -> AllocHandle {
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

    pub fn done_loading(game: &mut Game, world: AllocHandle) {
        game.worlds.push(world.deref());
    }
}

pub enum LoadingStage {
    Early,
    Loading,
    Finished
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}