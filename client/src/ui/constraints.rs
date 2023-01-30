use game::util::types::Vector2;
use crate::ui::layout::LayoutConstraint;

pub struct PixelConstraint {
    pixels: u32
}

pub struct PercentConstraint {
    percent: f32
}

impl PixelConstraint {
    pub fn new(pixels: u32) -> Self {
        return Self {
            pixels
        }
    }
}

impl PercentConstraint {
    pub fn new(percent: f32) -> Self {
        return Self {
            percent
        }
    }
}

impl LayoutConstraint for PixelConstraint {
    fn constriction(&self, parent: &u32) -> u32 {
        return parent - self.pixels as u32;
    }
}

impl LayoutConstraint for PercentConstraint {
    fn constriction(&self, parent: &u32) -> u32 {
        return (*parent as f32 * self.percent) as u32;
    }
}