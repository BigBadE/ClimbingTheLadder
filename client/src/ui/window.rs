use crate::ui::components::component::UIComponent;

pub struct UIWindow {
    content: UIComponent
}

impl UIWindow {
    pub fn new(content: UIComponent) -> Self {
        return Self {
            content
        }
    }

    pub fn update(&mut self) {
        //self.content.update();
    }
}