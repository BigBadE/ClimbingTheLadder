use instant::Instant;
use winit::event::{ElementState, KeyboardInput, MouseButton};
use crate::display::window::{Context, GameWindow};
use crate::renderer::renderer::Renderer;
use crate::ui::manager::UIManager;
use crate::display;
use core::rendering::renderable::Renderable;
use core::Game;

pub struct Client {
    next_update: Instant,
    window: GameWindow,
    game: Game,
    ui_manager: UIManager,
    renderer: Renderer,
}

impl Client {
    pub fn new(window: display::window::GameWindow, game: Game) -> Self {
        return Self {
            next_update: Instant::now() + game.settings.updates_per_second,
            window,
            game,
            renderer: Renderer::new(),
            ui_manager: UIManager::new(),
        };
    }
}

impl Context for Client {
    fn render(&mut self) {
        self.renderer.render(&mut self.window, &[self.game.render(), self.ui_manager.render()]);
    }

    fn update(&mut self) {
        self.next_update += self.game.settings.updates_per_second;
        self.game.update();
    }

    fn resize(&mut self, size: (u32, u32)) {
        self.ui_manager.resize(size);
    }

    fn key_input(&mut self, input: &KeyboardInput) {
        todo!()
    }

    fn mouse_input(&mut self, button: &MouseButton, state: &ElementState) {
        todo!()
    }

    fn cursor_move(&mut self, position: (f64, f64)) { self.ui_manager.cursor_pos = position; }

    fn update_time(&mut self) -> Instant {
        return self.next_update;
    }

    fn rendering_time(&mut self, last_update: Instant) -> Instant {
        return last_update + self.window.settings.frames_per_second;
    }
}