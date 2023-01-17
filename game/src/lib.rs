use std::borrow::Borrow;
use std::sync::{mpsc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use anyhow::Error;
use tokio::runtime::Runtime;
use tokio::task::{JoinHandle, JoinSet};
use crate::resources::ContentLoader;
use crate::mods::mod_manager::ModManager;
use crate::mods::mods::GameMod;
use crate::rendering::mesh::{FrameData, Mesh};
use crate::rendering::renderable::Renderable;
use crate::resources::resource_manager::ResourceManager;
use crate::settings::Settings;
use crate::util::runtime_factory::RuntimeFactory;
use crate::util::task_manager::TaskManager;
use crate::world::world::World;

pub mod mods;
pub mod rendering;
pub mod resources;
pub mod util;
pub mod world;
pub mod settings;

pub struct Game {
    pub settings: Settings,
    pub task_manager: TaskManager,
    resource_manager: ResourceManager,
    mods: ModManager,
    updater: Sender<()>
}

impl Game {
    pub async fn new(mods: JoinSet<Result<GameMod, Error>>, content: Box<dyn ContentLoader + Send>,
                     runtime_factory: Box<dyn RuntimeFactory>) -> Self {
        let settings = Settings::new();
        let mut task_manager = TaskManager::new(runtime_factory.spawn(), runtime_factory.spawn());
        let mut resource_manager = ResourceManager::new();
        task_manager.queue(true, resource_manager.load_types(content));
        let (sender, receiver): (Sender<()>, Receiver<()>) = mpsc::channel();

        runtime_factory.spawn().spawn(Self::update(receiver));
        return Self {
            settings,
            task_manager,
            resource_manager,
            mods: ModManager::new(mods),
            updater: sender
        };
    }

    pub fn notify_update(&self) {
        self.updater.send(()).unwrap();
    }

    pub async fn update(update: Receiver<()>) {
        let mut worlds: Vec<World> = Vec::new();
        loop {
            if update.recv().is_err() {
                error!("Error on update channel");
                return;
            }

            for world in &mut worlds {
                world.update();
            }
        }
    }
}

impl Renderable for Game {
    //This is run on the main thread instead of the update thread!
    fn data(&self) -> Vec<&Mesh> {
        todo!()
    }

    fn render(&self) -> FrameData {
        todo!()
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}