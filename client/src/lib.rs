
#[cfg(target_arch = "wasm32")]
use include_dir::{Dir, include_dir};

#[cfg(target_arch = "wasm32")]
use tokio::runtime::Builder;

#[cfg(target_arch = "wasm32")]
use tokio::task::JoinSet;

#[cfg(target_arch = "wasm32")]
use game::Game;

#[cfg(target_arch = "wasm32")]
use crate::client::Client;

#[cfg(target_arch = "wasm32")]
use crate::display::window::GameWindow;

#[cfg(target_arch = "wasm32")]
use crate::resources::web_loader::WebLoader;

#[cfg(target_arch = "wasm32")]
use game::util::task_manager;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
static RESOURCES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../resources");

pub mod debug;
pub mod display;
pub mod input;
pub mod resources;
pub mod renderer;
pub mod ui;
pub mod client;
pub mod settings;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[cfg(target_arch = "wasm32")]
pub async fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    let cpu_runtime = Builder::new_current_thread()
        .thread_name("ctl-cpu-worker")
        .thread_stack_size(3 * 1024 * 1024)
        .build().unwrap();

    let io_runtime = Builder::new_current_thread()
        .thread_name("ctl-io-worker")
        .thread_stack_size(3 * 1024 * 1024)
        .build().unwrap();

    GameWindow::run(Game::new(JoinSet::new(), Box::new(WebLoader::new(RESOURCES)),
                              TaskManager::new(cpu_runtime, io_runtime))).await;
}