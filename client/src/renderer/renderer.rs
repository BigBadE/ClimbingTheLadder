use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use wgpu::{Color, CommandEncoderDescriptor, Device, IndexFormat, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, SurfaceError, TextureViewDescriptor};
use game::error;
use game::rendering::GameTexture;
use game::rendering::mesh::{FrameData, Mesh};
use game::rendering::renderer::Renderer;
use crate::display::window::GameWindow;
use crate::renderer::rendering_data::RenderingData;
use crate::renderer::shaders::SHADER_MANAGER;

lazy_static! {
    pub static ref RENDERER: Mutex<GameRenderer> = Mutex::new(GameRenderer::new());
}

pub struct GameRenderer {
    last_id: u64,
    device: Option<Arc<Mutex<Device>>>,
    queue: Option<Arc<Queue>>,
    rendering: HashMap<u64, RenderingData>
}

impl GameRenderer {
    fn new() -> Self {
        return Self {
            last_id: 0,
            device: None,
            queue: None,
            rendering: HashMap::new()
        }
    }

    pub(crate) fn init(&mut self, device: Arc<Mutex<Device>>, queue: Arc<Queue>) {
        self.device = Some(device);
        self.queue = Some(queue);
    }

    pub fn render(&self, window: &mut GameWindow) -> Result<(), SurfaceError> {
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
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            for data in self.rendering.values() {
                match shaders.shaders.get(&data.shader) {
                    Some(shader) => {
                        render_pass.set_pipeline(&shader.0);
                        render_pass.set_bind_group(0, &data.bind_group, &[]);
                        render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(data.index_buffer.slice(..), IndexFormat::Uint16);
                        render_pass.draw_indexed(
                            0..data.index_buffer.size() as u32 / 2, 0, 0..1);
                    },
                    None => {
                        error!("No loaded shader named {}. Loaded: {:?}", data.shader,
                            shaders.shaders.keys());
                        continue;
                    }
                }
            }
        }

        // submit will accept anything that implements IntoIter
        window.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }

    pub fn push(&mut self, mesh: Arc<Mesh>, texture: Arc<dyn GameTexture>, data: FrameData) -> u64 {
        let id = self.last_id;
        self.last_id += 1;
        self.rendering.insert(id, RenderingData::new(
            self.device.as_ref().unwrap().lock().unwrap().deref(), self.queue.as_ref().unwrap(), mesh, texture, data));
        return id;
    }

    pub fn update(&mut self, id: u64, data: FrameData) {
        self.rendering.get_mut(&id).unwrap().update(data);
    }

    pub fn clear(&mut self, id: u64) {
        self.rendering.remove(&id);
    }
}

lazy_static! {
    pub static ref RENDERER_REF: Arc<dyn Renderer> = Arc::new(RendererRef::new());
}

pub struct RendererRef {}

impl RendererRef {
    fn new() -> Self {
        return Self {};
    }
}

impl Renderer for RendererRef {
    fn push(&self, mesh: Arc<Mesh>, texture: Arc<dyn GameTexture>, data: FrameData) -> u64 {
        return RENDERER.lock().unwrap().push(mesh, texture, data);
    }

    fn update(&self, id: u64, data: FrameData) {
        RENDERER.lock().unwrap().update(id, data);
    }

    fn clear(&self, id: u64) {
        RENDERER.lock().unwrap().clear(id);
    }
}