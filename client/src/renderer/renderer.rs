use std::collections::HashMap;
use wgpu::{Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
           RenderPassDescriptor, SurfaceError, TextureViewDescriptor};
use game::rendering::mesh::{FrameData, Mesh};
use game::rendering::renderer::Renderer;
use game::resources::content_pack::ContentPack;
use crate::display::window::GameWindow;
use crate::renderer::shaders::ShaderManager;

pub struct GameRenderer {
    last_id: u64,
    shaders: ShaderManager,
    rendering: HashMap<u64, (Mesh, FrameData)>
}

impl GameRenderer {
    pub fn new() -> Self {
        return Self {
            last_id: 0,
            shaders: ShaderManager::new(),
            rendering: HashMap::new()
        }
    }

    pub fn render(&self, window: &mut GameWindow) -> Result<(), SurfaceError> {
        let output = window.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = window.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0
                        }),
                        store: true
                    }
                })],
                depth_stencil_attachment: None
            });

            for (mesh, data) in self.rendering.values() {
                render_pass.set_pipeline(&self.shaders.shaders.get(&mesh.shader).unwrap().0);
                render_pass.draw(
                    0..mesh.vertexes.len() as u32, 0..mesh.vertexes.len() as u32/3);
            }
        }

        // submit will accept anything that implements IntoIter
        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }

    pub fn load_content(&mut self, window: &mut GameWindow, content: Box<dyn ContentPack>) {
        for (name, source) in content.shaders() {
            self.shaders.load(window, name, source);
        }
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