//No main in WASM
#![cfg_attr(target_arch = "wasm32", no_main)]

use std::env;
use std::sync::{Arc, Mutex};
use tokio::runtime::Builder;
use game::{Game, hashmap};
use game::resources::resource_manager::ResourceManager;
use game::util::register::GenericRegister;
use game::util::task_manager::TaskManager;
use crate::display::window::GameWindow;
use crate::input::manager::KeyAction;
use crate::mods::mod_loader::ModLoader;
use crate::renderer::assets::AssetReferer;
use crate::renderer::renderer::RENDERER_REF;
use crate::resources::desktop_loader::DesktopLoader;

pub mod debug;
pub mod display;
pub mod input;
pub mod mods;
pub mod renderer;
pub mod resources;
pub mod ui;
pub mod client;
pub mod settings;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    env_logger::init();

    let cpu_runtime = Builder::new_multi_thread()
        .thread_name("ctl-cpu-worker")
        .thread_stack_size(3 * 1024)
        .build().unwrap();

    let io_runtime = Builder::new_multi_thread()
        .worker_threads(10)
        .thread_name("ctl-io-worker")
        .thread_stack_size(3 * 1024)
        .build().unwrap();

    let main_runtime = Builder::new_current_thread()
        .thread_name("ctl-main")
        .thread_stack_size(3 * 1024 * 1024)
        .build().unwrap();

    let mut directory = env::current_dir().unwrap().join("resources");
    if !directory.exists() {
        directory = env::current_dir().unwrap().join("../resources");
        if !directory.exists() {
            panic!("Couldn't find resources directory!");
        }
    }

    let content = Box::new(DesktopLoader::new(directory.clone()));
    let task_manager = TaskManager::new(cpu_runtime.handle().clone(), io_runtime.handle().clone());
    let resource_manager = Arc::new(Mutex::new(
        ResourceManager::new(Box::new(ModLoader::new()), ModLoader::get_mods(directory, cpu_runtime.handle()),
                             Box::new(AssetReferer::new()), RENDERER_REF.clone())));
    let game = Game::new(resource_manager, task_manager,
                         hashmap!("keyaction" => Arc::new(GenericRegister::<KeyAction>::from())));
    GameWindow::run(game, content, main_runtime);
}