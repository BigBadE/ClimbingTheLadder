pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

pub struct Vector2Int {
    pub x: u32,
    pub y: u32
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Rectangle {
    pub position: Vector2Int,
    pub size: Vector2Int
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        return Vector2 {
            x,
            y
        }
    }
}

impl Vector2Int {
    pub fn new(x: u32, y: u32) -> Self {
        return Vector2Int {
            x,
            y
        }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vector3 {
            x,
            y,
            z
        }
    }
}

impl Rectangle {
    pub fn new(size: Vector2Int, position: Vector2Int) -> Self {
        return Rectangle {
            size,
            position
        }
    }
}