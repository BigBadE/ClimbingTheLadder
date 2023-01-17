//No main in WASM
#![cfg_attr(target_arch = "wasm32", no_main)]

use tokio::runtime::Builder;
use game::Game;
use game::util::task_manager::TaskManager;
use crate::client::Client;
use crate::display::window::GameWindow;
use crate::mods::mod_loader::load_mods;
use crate::resources::desktop_loader::DesktopLoader;

pub mod debug;
pub mod display;
pub mod input;
pub mod mods;
pub mod resources;
pub mod renderer;
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

    main_runtime.block_on(
        GameWindow::run(Game::new(load_mods(&io_runtime), Box::new(DesktopLoader::new()),
                                  TaskManager::new(cpu_runtime, io_runtime)), Client::new));
}