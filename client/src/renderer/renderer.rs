use std::collections::HashMap;
use anyhow::Error;
use wgpu::{Color, CommandEncoderDescriptor, LoadOp, RenderPassColorAttachment, RenderPassDescriptor, TextureViewDescriptor};
use game::rendering::mesh::{FrameData, Mesh};
use game::rendering::renderer::Renderer;
use crate::display::window::GameWindow;

pub struct GameRenderer {
    last_id: u64,
    rendering: HashMap<u64, (Mesh, FrameData)>
}

impl GameRenderer {
    pub fn new() -> Self {
        return Self {
            last_id: 0,
            rendering: HashMap::new()
        }
    }

    pub fn render(&self, window: &mut GameWindow) -> Result<(), wgpu::SurfaceError> {
        let output = window.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = window.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        for (mesh, data) in self.rendering.values() {

        }

        return Ok(());
    }
}

impl Renderer for GameRenderer {
    fn push(&mut self, mesh: Mesh, data: FrameData) -> u64 {
        let id = self.last_id;
        self.last_id += 1;
        self.rendering.insert(id, (mesh, data));
        return id;
    }

    fn update(&mut self, id: u64, data: FrameData) {
        let mesh = self.rendering.remove(&id).unwrap().0;
        self.rendering.insert(id, (mesh, data));
    }

    fn clear(&mut self, id: u64) {
        self.rendering.remove(&id);
    }
}