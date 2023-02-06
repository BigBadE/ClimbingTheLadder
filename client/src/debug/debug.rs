use winit::event::ElementState;

fn toggle_console(state: &ElementState) {
    if state != &ElementState::Pressed {
        return;
    }
}