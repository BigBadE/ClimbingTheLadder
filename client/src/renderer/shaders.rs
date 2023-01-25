use std::collections::HashMap;
use wgpu::{BlendState, ColorTargetState, ColorWrites, Face, FragmentState, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, VertexState};
use crate::display::window::GameWindow;

pub struct ShaderManager {
    pub shaders: HashMap<String, (RenderPipeline, ShaderModule)>
}

impl ShaderManager {
    pub fn new() -> Self {
        return Self {
            shaders: HashMap::new()
        }
    }

    pub fn load(&mut self, window: &GameWindow, name: String, source: String) {
        let shader = window.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(name.as_str()),
            source: ShaderSource::Wgsl(source.into())
        });
        self.shaders.insert(name.clone(), (Self::get_pipeline(window, &shader), shader));
    }

    pub fn get_pipeline(window: &GameWindow, shader: &ShaderModule) -> RenderPipeline {
        let layout = window.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });
        return window.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: window.config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL
                })]
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });
    }
}