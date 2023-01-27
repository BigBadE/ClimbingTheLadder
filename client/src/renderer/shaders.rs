use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use wgpu::{BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, SurfaceConfiguration, VertexState};
use game::Game;
use game::resources::content_pack::ContentPack;
use game::util::alloc_handle::AllocHandle;

pub struct ShaderManager {
    pub shaders: HashMap<String, (RenderPipeline, ShaderModule)>,
    pub loaded_ui_shaders: bool
}

lazy_static! {
    pub static ref SHADER_MANAGER: Mutex<ShaderManager> = Mutex::new(ShaderManager::new());
}

impl ShaderManager {
    fn new() -> Self {
        return Self {
            shaders: HashMap::new(),
            loaded_ui_shaders: false
        };
    }

    pub async fn load(device: Arc<Mutex<Device>>, config: SurfaceConfiguration, load_first: bool, content: Box<dyn ContentPack>) -> AllocHandle {
        let device = device.lock().unwrap();
        let device = device.deref();
        let mut shaders = HashMap::new();
        let found_shaders;
        if load_first {
            found_shaders = content.load_first_shaders();
        } else {
            found_shaders = content.shaders();
        }

        for (name, source) in found_shaders {
            let shader = device.create_shader_module(ShaderModuleDescriptor {
                label: Some(name.as_str()),
                source: ShaderSource::Wgsl(source.into()),
            });
            shaders.insert(name.clone(), (Self::get_pipeline(device, &config, &shader), shader));
        }
        let mut manager = SHADER_MANAGER.lock().unwrap();
        for (name, shader) in shaders {
            manager.shaders.insert(name, shader);
        }

        if manager.shaders.contains_key("shader") {
            manager.loaded_ui_shaders = true;
        }

        return AllocHandle::empty();
    }

    pub fn get_pipeline(device: &Device, config: &SurfaceConfiguration, shader: &ShaderModule) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        return device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
    }
}