use game::util::types::{Rectangle, Vector2Int};

pub trait LayoutConstraint {
    fn constriction(&self, parent: &u32) -> u32;
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

    fn get_position(&self, parent: &Rectangle) -> Vector2Int {
        return Vector2Int::new(self.x.constriction(&parent.size.x), self.y.constriction(&parent.size.y));
    }

    fn get_size(&self, parent: &Rectangle) -> Vector2Int {
        return Vector2Int::new(self.width.constriction(&parent.size.x), self.height.constriction(&parent.size.y));
    }
}