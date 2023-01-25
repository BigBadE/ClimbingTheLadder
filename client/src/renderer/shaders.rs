use std::collections::HashMap;
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};
use crate::display::window::GameWindow;

pub struct ShaderManager {
    pub shaders: HashMap<String, ShaderModule>
}

impl ShaderManager {
    pub fn new() -> Self {
        return Self {
            shaders: HashMap::new()
        }
    }

    pub fn load(&mut self, window: &GameWindow, name: String, source: String) {
        self.shaders.insert(name.clone(), window.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(name.as_str()),
            source: ShaderSource::Wgsl(source.into())
        }));
    }
}