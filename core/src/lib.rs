#![no_std]

extern crate alloc;

use tokio::runtime::Runtime;
use crate::rendering::mesh::Mesh;
use crate::rendering::renderable::Renderable;
use crate::settings::Settings;

pub mod rendering;
pub mod world;
pub mod settings;

pub struct Game {
    pub settings: Settings,
    cpu_runtime: Runtime,
    io_runtime: Runtime
}

impl Game {
    pub async fn new(cpu_runtime: Runtime, io_runtime: Runtime) -> Self {
        let settings = Settings::new();
        return Self {
            settings,
            cpu_runtime,
            io_runtime
        }
    }

    pub fn update(&mut self) {

    }
}

impl Renderable for Game {
    fn render<'a>(&mut self) -> &'a [Mesh] {
        todo!()
    }
}