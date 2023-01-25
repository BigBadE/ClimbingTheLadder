use crate::util::types::Vector3;

pub struct Mesh {
    pub shader: String,
    pub vertexes: Vec<Vertex>,
    pub indices: Vec<u32>
}

impl Mesh {
    pub fn new(shader: String) -> Self {
        return Self {
            shader,
            vertexes: Vec::new(),
            indices: Vec::new()
        }
    }
}

pub struct FrameData {
    pub offset: Vector3
}

impl FrameData {
    pub fn new() -> Self {
        return Self {
            offset: Vector3::new(0f32, 0f32, 0f32)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        return Self {
            position,
            tex_coords
        }
    }
}