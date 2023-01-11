pub struct Mesh {
    pub vertexes: Vec<Vertex>,
    pub indices: Vec<u32>
}

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}