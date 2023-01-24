use game::util::types::{Vector2, Rectangle};

pub trait LayoutConstraint {
    fn constriction(&self, parent: &Vector2) -> f32;
}

pub struct ComponentLayout {
    x: Box<dyn LayoutConstraint>,
    y: Box<dyn LayoutConstraint>,
    width: Box<dyn LayoutConstraint>,
    height: Box<dyn LayoutConstraint>
}

impl ComponentLayout {
    pub fn new(x: Box<dyn LayoutConstraint>, y: Box<dyn LayoutConstraint>,
        width: Box<dyn LayoutConstraint>, height: Box<dyn LayoutConstraint>) -> Self {
        return ComponentLayout {
            x,
            y,
            width,
            height
        }
    }

    fn get_position(&self, parent: &Rectangle) -> Vector2 {
        return Vector2::new(self.x.constriction(&parent.size), self.y.constriction(&parent.size));
    }

    fn get_size(&self, parent: &Rectangle) -> Vector2 {
        return Vector2::new(self.width.constriction(&parent.size), self.height.constriction(&parent.size));
    }
}