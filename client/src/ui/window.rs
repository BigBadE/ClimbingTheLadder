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
}