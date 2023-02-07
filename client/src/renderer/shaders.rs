use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::task::JoinSet;
use wgpu::{BindGroupLayoutEntry, ShaderStages, BindingType, TextureViewDimension, TextureSampleType, SamplerBindingType, BlendState, ColorTargetState, ColorWrites, Device, Face, FragmentState, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, SurfaceConfiguration, VertexState, VertexAttribute, vertex_attr_array};
use game::error;
use game::rendering::mesh::Vertex;
use game::util::alloc_handle::AllocHandle;
use crate::renderer::renderer::RENDERER;
use crate::resources::content_pack::ContentPack;

pub struct ShaderManager {
    pub shaders: HashMap<String, Shader>,
    pub loaded_ui_shaders: bool,
}

pub struct Shader {
    pub pipeline: RenderPipeline,
    pub module: ShaderModule,
    pub ui: bool
}

lazy_static! {
    pub static ref SHADER_MANAGER: Mutex<ShaderManager> = Mutex::new(ShaderManager::new());
    pub static ref BIND_LAYOUT: [BindGroupLayoutEntry; 2] = [
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ];
}

impl ShaderManager {
    fn new() -> Self {
        return Self {
            shaders: HashMap::new(),
            loaded_ui_shaders: false,
        };
    }

    pub fn get_shaders(first: bool, runtime: Handle, content: Box<dyn ContentPack>) -> JoinSet<(String, String)> {
        let mut loading = JoinSet::new();
        for shader in content.shaders(first) {
            loading.spawn_on(Self::load_shader(content.get_relative("shaders/", &shader), shader), &runtime);
        }
        return loading;
    }

    async fn load_shader(relative: String, shader: PathBuf) -> (String, String) {
        return match fs::read_to_string(shader) {
            Ok(shader) => (relative, shader),
            Err(error) => {
                error!("Error loading shader:\n{}", error);
                (relative, String::new())
            }
        };
    }

    pub async fn load(ui: bool, device: Arc<Mutex<Device>>, config: SurfaceConfiguration,
                      mut shaders: JoinSet<(String, String)>) -> AllocHandle {
        while let Some(result) = shaders.join_next().await {
            let (name, source) = match result {
                Ok(result) => result,
                Err(error) => {
                    error!("Failed to load shader:\n{}", error);
                    continue
                }
            };

            let device = device.lock().unwrap();
            let device = device.deref();

            let shader = device.create_shader_module(ShaderModuleDescriptor {
                label: Some(name.as_str()),
                source: ShaderSource::Wgsl(source.into()),
            });
            SHADER_MANAGER.lock().unwrap().shaders.insert(name.clone(), Shader::new(
                Self::get_pipeline(ui, device, &config, &shader), shader, ui));
        }

        return AllocHandle::empty();
    }

    pub fn get_pipeline(ui: bool, device: &Device, config: &SurfaceConfiguration, shader: &ShaderModule) -> RenderPipeline {
        let bind_group = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: BIND_LAYOUT.deref(),
            label: Some("Texture Bind Group Layout"),
        });
        let layout;
        if ui {
            layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group],
                push_constant_ranges: &[],
            });
        } else {
            layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group, &RENDERER.lock().unwrap().camera.as_ref().unwrap().camera_bind_group_layout],
                push_constant_ranges: &[],
            });
        }
        return device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Self::description()],
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

    const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl Shader {
    pub fn new(pipeline: RenderPipeline, module: ShaderModule, ui: bool) -> Self {
        return Self {
            pipeline,
            module,
            ui
        }
    }
}