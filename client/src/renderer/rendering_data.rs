use wgpu::{Buffer, BufferUsages, Device};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use game::rendering::mesh::{FrameData, Mesh};

pub struct RenderingData {
    pub vertex_buffer: Buffer,
    pub shader: String
}

impl RenderingData {
    pub fn new(device: &Device, mesh: Mesh, frame_data: FrameData) -> Self {
        return Self {
            vertex_buffer: device.create_buffer_init(
                &BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(mesh.vertexes.as_slice()),
                    usage: BufferUsages::VERTEX
                }
            ),
            shader: mesh.shader
        }
    }

    pub fn update(&mut self, data: FrameData) {

    }
}