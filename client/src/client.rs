use instant::Instant;
use tokio::runtime::Runtime;
use winit::event::{ElementState, KeyboardInput, ModifiersState, MouseButton};
use crate::display::window::GameWindow;
use crate::renderer::renderer::Renderer;
use crate::ui::manager::UIManager;
use core::rendering::renderable::Renderable;
use core::Game;

pub struct Client {
    next_update: Instant,
    window: GameWindow,
    game: Game,
    ui_manager: UIManager,
    renderer: Renderer
}

impl Client {
    pub fn new(window: GameWindow, game: Game) -> Self {
        return Self {
            next_update: Instant::now() + game.settings.updates_per_second,
            window,
            game,
            renderer: Renderer::new(),
            ui_manager: UIManager::new(),
        };
    }

    pub fn render(&mut self) {
        self.renderer.render(&mut self.window, &[self.game.data(), self.ui_manager.data()]);
    }

    pub fn update(&mut self) {
        self.next_update += self.game.settings.updates_per_second;
        self.game.notify_update();
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