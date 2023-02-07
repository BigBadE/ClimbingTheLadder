use std::string::ToString;
use cgmath::Vector3;
use lazy_static::lazy_static;
use macros::JsonLoadable;

#[derive(JsonLoadable, Clone, Debug, Default)]
pub struct Mesh {
    #[require_field]
    pub shader: String,
    pub vertexes: Vec<Vertex>,
    pub indices: Vec<u16>,
}

lazy_static! {
    pub static ref CUBE: Mesh = Mesh {
        shader: "shader".to_string(),
        vertexes: vec!(
            Vertex::new([0.0, 0.0, 0.0], [0.0, 0.0]), Vertex::new([1.0, 0.0, 0.0], [1.0, 0.0]),
            Vertex::new([0.0, 1.0, 0.0], [0.0, 1.0]), Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0])),
            indices: vec!(0, 1, 2, 1, 3, 2)
    };
}

impl Mesh {
    pub const fn new(shader: String) -> Self {
        return Self {
            shader,
            vertexes: Vec::new(),
            indices: Vec::new(),
        };
    }

    pub fn cube(shader: String) -> Self {
        let mut temp = CUBE.clone();
        temp.shader = shader;
        return temp;
    }
}

pub struct FrameData {
    pub offset: Vector3<f32>,
}

impl FrameData {
    pub fn new() -> Self {
        return Self {
            offset: Vector3::new(0.0, 0.0, 0.0)
        };
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, JsonLoadable, Default)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        return Self {
            position,
            tex_coords,
        };
    }
}