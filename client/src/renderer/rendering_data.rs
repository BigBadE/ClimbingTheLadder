use std::mem::size_of;
use std::ops::Deref;
use std::sync::Arc;
use core::num::NonZeroU32;
use wgpu::{BindGroup, Buffer, BufferUsages, Device, Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use game::rendering::GameTexture;
use game::rendering::mesh::{FrameData, Mesh};
use crate::renderer::shaders::BIND_LAYOUT;

pub struct RenderingData {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub texture: Texture,
    pub bind_group: BindGroup,
    pub shader: String
}

impl RenderingData {
    pub fn new(device: &Device, queue: &Queue, mesh: Arc<Mesh>, texture: Arc<dyn GameTexture>, frame_data: FrameData) -> Self {
        let size = Extent3d {
            width: texture.dimensions().0,
            height: texture.dimensions().1,
            depth_or_array_layers: 1
        };
        let loaded_texture = device.create_texture(&TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some(texture.name().as_str()),
            view_formats: &[TextureFormat::Rgba8UnormSrgb]
        });
        queue.write_texture(
            ImageCopyTexture {
                texture: &loaded_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All
            },
            texture.data(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * texture.dimensions().0),
                rows_per_image: NonZeroU32::new(texture.dimensions().1)
            },
            size
        );

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: BIND_LAYOUT.deref(),
                label: Some("Texture Bind Group Layout"),
            });
        let diffuse_texture_view = loaded_texture.create_view(&TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("Diffuse Bind Group"),
            }
        );
        return Self {
            vertex_buffer: device.create_buffer_init(
                &BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: Self::cast(mesh.vertexes.as_slice()),
                    usage: BufferUsages::VERTEX,
                }
            ),
            index_buffer: device.create_buffer_init(
                &BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: Self::cast(mesh.indices.as_slice()),
                    usage: BufferUsages::INDEX,
                }
            ),
            texture: loaded_texture,
            bind_group,
            shader: mesh.shader.clone(),
        };
    }

    fn cast<A, B>(input: &[A]) -> &[B] {
        let new_len = core::mem::size_of_val(input) / size_of::<u8>();
        return unsafe { core::slice::from_raw_parts(input.as_ptr() as *const B, new_len) };
    }

    pub fn update(&mut self, data: FrameData) {}
}