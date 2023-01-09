//No main in WASM
#![cfg_attr(target_arch = "wasm32", no_main)]

use std::future;
use std::task::Poll;
use tokio::runtime::Builder;
use core::Game;
use crate::client::Client;
use crate::display::window::GameWindow;

pub mod display;
pub mod input;
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

    main_runtime.block_on(GameWindow::run(Game::new(cpu_runtime, io_runtime), Client::new));
}