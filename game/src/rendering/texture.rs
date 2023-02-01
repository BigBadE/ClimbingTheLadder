use crate::rendering::GameTexture;

pub struct ColorTexture {
    color: [u8; 4]
}

impl GameTexture for ColorTexture {
    fn dimensions(&self) -> (u32, u32) {
        return (1, 1)
    }

    fn name(&self) -> String {
        return format!("Color: {:?}", self.color);
    }

    fn data(&self) -> &[u8] {
        return &self.color;
    }
}

impl ColorTexture {
    pub fn new(color: [u8; 4]) -> Self {
        return Self {
            color
        };
    }
}