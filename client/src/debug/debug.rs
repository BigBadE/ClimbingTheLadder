use winit::event::{ElementState, VirtualKeyCode};
use crate::Client;
use crate::input::manager::InputManager;
use crate::ui::manager::UIManager;

pub fn init(ui_manager: &mut UIManager, inputs: &mut InputManager) {
    inputs.add_action((VirtualKeyCode::Grave, 0), ("Toggle Console", toggle_console));
    //ui_manager.open(UIW);
}

fn toggle_console(state: &ElementState) {
    if state != &ElementState::Pressed {
        return;
    }


}