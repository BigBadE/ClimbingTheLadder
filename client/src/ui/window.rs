use crate::ui::components::component::UIComponent;

pub struct UIWindow {
    _content: UIComponent
}

impl UIWindow {
    pub fn new(content: UIComponent) -> Self {
        return Self {
            _content: content
        }
    }

    pub fn update(&mut self) {
        //self.content.update();
    }
}