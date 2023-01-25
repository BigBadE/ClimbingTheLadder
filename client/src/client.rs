use instant::Instant;
use wgpu::SurfaceError;
use winit::event::{ElementState, KeyboardInput, ModifiersState, MouseButton};
use crate::display::window::GameWindow;
use crate::renderer::renderer::GameRenderer;
use crate::ui::manager::UIManager;
use game::{error, Game};

pub struct Client {
    game: Game,
    window: GameWindow,
    next_update: Instant,
    ui_manager: UIManager,
    renderer: GameRenderer
}

impl Client {
    pub fn new(window: GameWindow, game: Game) -> Self {
        return Self {
            game,
            window,
            next_update: Instant::now(),
            renderer: GameRenderer::new(),
            ui_manager: UIManager::new()
        };
    }

    pub fn render(&mut self) -> bool {
        return match self.renderer.render(&mut self.window) {
            Ok(()) => false,
            // Reconfigure the surface if lost
            Err(SurfaceError::Lost) => {
                self.resize(self.window.size);
                false
            },
            // The system is out of memory, we should probably quit
            Err(SurfaceError::OutOfMemory) => true,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => {
                error!("{:?}", e);
                false
            }
        }
    }

    pub fn request_redraw(&self) {
        self.window.inner.request_redraw();
    }

    pub fn update(&mut self) {
        self.next_update += self.game.notify_update();
    }

    pub(crate) fn key_modifier_change(&mut self, modifiers: &ModifiersState) {
        self.window.modifiers = modifiers.bits();
    }

    pub(crate) fn resize(&mut self, size: (u32, u32)) {
        self.ui_manager.resize(size);
    }

    pub(crate) fn key_input(&mut self, input: &KeyboardInput) {
        match input.virtual_keycode {
            Some(keycode) => match self.window.settings.inputs.map(self.window.modifiers, keycode) {
                Some(function) => function(&input.state),
                None => {}
            },
            None => {}
        }
    }

    pub(crate) fn mouse_input(&mut self, button: &MouseButton, state: &ElementState) {
        match self.window.settings.inputs.map_mouse(button) {
            Some(function) => function(state),
            None => {}
        }
    }

    pub(crate) fn cursor_move(&mut self, position: (f64, f64)) { self.ui_manager.cursor_pos = position; }

    pub(crate) fn update_time(&mut self) -> Instant {
        return self.next_update;
    }

    pub(crate) fn rendering_time(&mut self, last_update: Instant) -> Instant {
        return last_update + self.window.settings.frames_per_second;
    }
}