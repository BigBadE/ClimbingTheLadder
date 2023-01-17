pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Rectangle {
    pub position: Vector2,
    pub size: Vector2
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        return Vector2 {
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
    pub fn new(size: Vector2, position: Vector2) -> Self {
        return Rectangle {
            size,
            position
        }
    }
}