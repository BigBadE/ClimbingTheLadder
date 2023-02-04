use cgmath::Vector2;
use game::util::Rectangle;

pub trait LayoutConstraint {
    fn constriction(&self, parent: &u32) -> u32;
}

pub struct ComponentLayout {
    _x: Box<dyn LayoutConstraint>,
    _y: Box<dyn LayoutConstraint>,
    _width: Box<dyn LayoutConstraint>,
    _height: Box<dyn LayoutConstraint>
}

impl ComponentLayout {
    pub fn new(x: Box<dyn LayoutConstraint>, y: Box<dyn LayoutConstraint>,
        width: Box<dyn LayoutConstraint>, height: Box<dyn LayoutConstraint>) -> Self {
        return ComponentLayout {
            _x: x,
            _y: y,
            _width: width,
            _height: height
        }
    }

    fn _get_position(&self, parent: &Rectangle) -> Vector2<u32> {
        return Vector2::new(self._x.constriction(&parent.size.x), self._y.constriction(&parent.size.y));
    }

    fn _get_size(&self, parent: &Rectangle) -> Vector2<u32> {
        return Vector2::new(self._width.constriction(&parent.size.x), self._height.constriction(&parent.size.y));
    }
}