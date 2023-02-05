use winit::event::{ElementState, VirtualKeyCode};
use crate::input::manager::InputManager;
use crate::ui::manager::UIManager;

fn toggle_console(state: &ElementState) {
    if state != &ElementState::Pressed {
        return;
    }
}