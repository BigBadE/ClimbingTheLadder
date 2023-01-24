use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::sync::Mutex;
use std::time::Duration;
use anyhow::Error;
use tokio::task::JoinSet;
use crate::resources::ContentLoader;
use crate::mods::mod_manager::ModManager;
use crate::mods::mods::GameMod;
use crate::register::ThingRegister;
use crate::resources::resource_manager::ResourceManager;
use crate::settings::Settings;
use crate::util::task_manager::TaskManager;
use crate::world::world::World;

pub mod mods;
pub mod register;
pub mod rendering;
pub mod resources;
pub mod util;
pub mod world;
pub mod settings;

static GAME_MUTEX: Mutex<MaybeUninit<Game>> = Mutex::new(MaybeUninit::uninit());

pub struct Game {
    pub settings: Settings,
    pub task_manager: TaskManager,
    worlds: Vec<World>,
    resource_manager: ResourceManager,
    mods: ModManager,
    registerer: HashMap<&'static str, Box<dyn ThingRegister + Send + Sync>>
}

impl Game {
    pub async fn init(mods: JoinSet<Result<GameMod, Error>>, content: Box<dyn ContentLoader + Send>,
                     mut task_manager: TaskManager) {
        //Hold the lock until inited.
        let locked = GAME_MUTEX.lock();

        let settings = Settings::new();
        let resource_manager = ResourceManager::new();
        task_manager.queue_after(true, ResourceManager::load_types(content), ResourceManager::finish_loading);

        let output = Self {
            settings,
            task_manager,
            resource_manager,
            mods: ModManager::new(mods),
            worlds: Vec::new(),
            registerer: HashMap::new()
        };

        locked.unwrap().write(output);
    }

    pub async fn create_world(&mut self) {
        self.worlds.push(World::new(&self.task_manager, self.registerer.get("world").unwrap()));
    }

    pub fn notify_update() -> Duration {
        unsafe {
            let mut game = GAME_MUTEX.lock().unwrap();
            let game = game.assume_init_mut();
            //Poll tasks
            let mut polled = game.task_manager.poll();
            //If one task is finished, poll the next.
            while polled.1.is_some() {
                polled.1.unwrap().call(game);
                polled = game.task_manager.poll();
            }

            //Skip update if it's running a long task.
            if polled.0 == false {
                return game.settings.updates_per_second;
            }

            for world in &mut game.worlds {
                world.update();
            }
            return game.settings.updates_per_second;
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}