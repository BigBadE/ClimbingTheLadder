use crate::util::types::Vector3;

pub struct Mesh {
    pub vertexes: Vec<Vertex>,
    pub indices: Vec<u32>
}

pub struct FrameData {
    pub offset: Vector3
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}