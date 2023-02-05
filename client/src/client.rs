use instant::Instant;
use wgpu::SurfaceError;
use winit::event::{ElementState, KeyboardInput, ModifiersState, MouseButton};
use crate::display::window::GameWindow;
use crate::renderer::renderer::{RENDERER, RENDERER_REF};
use crate::ui::manager::UIManager;
use game::{error, Game};
use game::resources::content_pack::ContentPack;
use crate::renderer::shaders::{SHADER_MANAGER, ShaderManager};

pub struct Client {
    game: Game,
    window: GameWindow,
    next_update: Instant,
    ui_manager: UIManager
}

impl Client {
    pub fn new(window: GameWindow, mut game: Game, content: Box<dyn ContentPack>) -> Self {
        game.task_manager.queue(false, ShaderManager::load(
            window.device.clone(), window.config.clone(), true, content.clone_boxed()));
        game.task_manager.queue(false, ShaderManager::load(
            window.device.clone(), window.config.clone(), false, content.clone_boxed()));
        //You may be wondering why go through this load function.
        //It's because if you don't, lazy_static will re-initialize the static for no reason.
        game.resource_manager.lock().unwrap().asset_manager.load(
            &mut game.task_manager, content.clone_boxed());

        return Self {
            game,
            window,
            next_update: Instant::now(),
            ui_manager: UIManager::new(&RENDERER_REF)
        };
    }

    pub fn render(&mut self) -> bool {
        if !SHADER_MANAGER.lock().unwrap().loaded_ui_shaders {
            return false;
        }

        let result = RENDERER.lock().unwrap().render(&mut self.window);
        return match result {
            Ok(()) => false,
            // Reconfigure the surface if lost
            Err(SurfaceError::Lost) => {
                self.resize(self.window.size);
                false
            }
            // The system is out of memory, we should probably quit
            Err(SurfaceError::OutOfMemory) => true,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => {
                error!("{:?}", e);
                false
            }
        };
    }

    pub fn request_redraw(&self) {
        self.window.inner.request_redraw();
    }

    pub async fn update(&mut self) {
        self.next_update += self.game.notify_update().await;
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
                Some(function) => function(&mut self.game, &input.state),
                None => {}
            },
            None => {}
        }
    }

    pub(crate) fn mouse_input(&mut self, button: &MouseButton, state: &ElementState) {
        match self.window.settings.inputs.map_mouse(button) {
            Some(function) => function(&mut self.game, state),
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