use std::collections::HashMap;
use wgpu::{Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, SurfaceError, TextureViewDescriptor};
use game::error;
use game::rendering::mesh::{FrameData, Mesh};
use game::rendering::renderer::Renderer;
use crate::display::window::GameWindow;
use crate::renderer::shaders::SHADER_MANAGER;

pub struct GameRenderer {

}

impl GameRenderer {
    pub fn render(window: &mut GameWindow, data: &Box<dyn Renderer>) -> Result<(), SurfaceError> {
        let output = window.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = window.device.lock().unwrap().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let shaders = SHADER_MANAGER.lock().unwrap();
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

            for (mesh, _data) in data.get_data().values() {
                match shaders.shaders.get(&mesh.shader) {
                    Some(shader) => render_pass.set_pipeline(&shader.0),
                    None => {
                        error!("No loaded shader named {}. Loaded: {:?}", mesh.shader,
                            shaders.shaders.keys());
                        continue
                    }
                }
                render_pass.draw(
                    0..mesh.vertexes.len() as u32, 0..mesh.vertexes.len() as u32/3);
            }
        }

        // submit will accept anything that implements IntoIter
        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }
}

pub struct RenderData {
    last_id: u64,
    rendering: HashMap<u64, (Mesh, FrameData)>
}

impl RenderData {
    pub fn new() -> Self {
        return Self {
            last_id: 0,
            rendering: HashMap::new()
        }
    }
}

impl Renderer for RenderData {
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

    fn get_data(&self) -> &HashMap<u64, (Mesh, FrameData)> {
        return &self.rendering;
    }
}