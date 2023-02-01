use std::string::ToString;
use json::JsonValue;
use lazy_static::lazy_static;
use crate::error;
use crate::util::types::Vector3;

#[derive(Clone)]
pub struct Mesh {
    pub shader: String,
    pub vertexes: Vec<Vertex>,
    pub indices: Vec<u16>,
}

lazy_static! {
    pub static ref CUBE: Mesh = Mesh {
        shader: "shader".to_string(),
        vertexes: vec!(
            /*Left*/Vertex::new([0f32, 0f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 0f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 1f32, 0f32], [0f32, 1f32]), Vertex::new([0f32, 1f32, 0f32], [0f32, 0f32]),
            /*Back*/Vertex::new([1f32, 0f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 0f32, 1f32], [0f32, 0f32]), Vertex::new([1f32, 1f32, 1f32], [1f32, 1f32]), Vertex::new([0f32, 0f32, 0f32], [0f32, 0f32]),
            /*Right*/Vertex::new([1f32, 0f32, 1f32], [0f32, 0f32]), Vertex::new([1f32, 1f32, 1f32], [0f32, 0f32]), Vertex::new([0f32, 1f32, 1f32], [0f32, 0f32]), Vertex::new([0f32, 0f32, 1f32], [0f32, 0f32]),
            /*Front*/Vertex::new([0f32, 0f32, 1f32], [0f32, 0f32]), Vertex::new([0f32, 1f32, 1f32], [0f32, 0f32]), Vertex::new([0f32, 1f32, 0f32], [0f32, 0f32]), Vertex::new([0f32, 0f32, 0f32], [0f32, 0f32]),
            /*Top*/Vertex::new([0f32, 1f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 1f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 1f32, 1f32], [0f32, 0f32]), Vertex::new([0f32, 1f32, 1f32], [0f32, 0f32]),
            /*Bottom*/Vertex::new([0f32, 0f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 0f32, 0f32], [0f32, 0f32]), Vertex::new([1f32, 0f32, 1f32], [0f32, 0f32]), Vertex::new([0f32, 0f32, 1f32], [0f32, 0f32])),
        indices: vec!(/*0*/ 0, 1, 2, 2, 1, 3, /*1*/ 4, 5, 6, 6, 5, 7, /*2*/ 8, 9, 10, 10, 9, 11,
                    /*3*/ 12, 13, 14, 14, 13, 15, /*4*/ 16, 17, 18, 18, 17, 19, /*5*/ 20, 21, 22, 22, 21, 23)
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

    pub fn load(model: JsonValue) -> Self {
        let mut returning = Self::new(model["shader"].to_string());
        match &model["vertexes"] {
            JsonValue::Array(array) => {
                for value in array {
                    returning.vertexes.push(Vertex::load(value));
                }
            }
            _ => error!("Expected array for vertexes")
        }
        match &model["indices"] {
            JsonValue::Array(array) => {
                for value in array {
                    match value {
                        JsonValue::Number(number) => returning.indices.push(f64::from(*number) as u16),
                        _ => error!("Expected vertex in JSON model:\n{}", model)
                    }
                }
            }
            _ => error!("Expected array for vertexeindexess")
        }
        return returning;
    }

    pub fn cube(shader: String) -> Self {
        let mut temp = CUBE.clone();
        temp.shader = shader;
        return temp;
    }
}

pub struct FrameData {
    pub offset: Vector3,
}

impl FrameData {
    pub fn new() -> Self {
        return Self {
            offset: Vector3::new(0f32, 0f32, 0f32)
        };
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        return Self {
            position,
            tex_coords,
        };
    }

    pub fn load(loading: &JsonValue) -> Self {
        return Self {
            position: Self::load_array(&loading["pos"]),
            tex_coords: Self::load_array(&loading["tex"])
        }
    }

    fn load_array<const L: usize>(from: &JsonValue) -> [f32; L] {
        return match from {
            JsonValue::Array(array) => {
                let mut loading = [0f32; L];
                for i in 0..L {
                    loading[i] = match array.get(i).unwrap() {
                        JsonValue::Number(number) => f32::from(*number),
                        _ => {
                            error!("Unknown type, expected number:\n{}", from);
                            return [0f32; L];
                        }
                    }
                }
                loading
            },
            _ => {
                error!("Expected array, found:\n{}", from);
                [0f32; L]
            }
        }
    }
}