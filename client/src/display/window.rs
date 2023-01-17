use std::future::Future;
use instant::Instant;
use tokio::runtime::Runtime;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, ModifiersState, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use core::Game;
use game::Game;
use crate::client::Client;
use crate::settings::GameSettings;

pub struct GameWindow {
    pub settings: GameSettings,
    pub modifiers: u32,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>
}

impl GameWindow {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        //Make sure it's >0 or it may crash
        let size = PhysicalSize::new(size.width.max(1), size.height.max(1));

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        return Self {
            settings: GameSettings::new(),
            modifiers: 0,
            surface,
            device,
            queue,
            config,
            size
        };
    }

    pub async fn run(game: impl Future<Output=Game>, update_runtime: Runtime) {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new().build(&event_loop).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(450, 400));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let id = window.id();
        let window = GameWindow::new(&window).await;
        let mut context = Client::new(window, game.await);
        let mut next_frame = context.rendering_time(Instant::now());
        event_loop.run(move |ev, _, control_flow| {
            let rendering;
            //Figure out if we're updating the game or rendering the game
            if next_frame > context.update_time() {
                rendering = true;
                next_frame = context.rendering_time(next_frame);
                *control_flow = ControlFlow::WaitUntil(next_frame);
            } else {
                rendering = false;
                *control_flow = ControlFlow::WaitUntil(context.update_time());
            }

            match ev {
                Event::WindowEvent {
                    ref event, window_id
                } if window_id == id =>
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        WindowEvent::Resized(size) => {
                            context.resize((size.width, size.height));
                        }
                        WindowEvent::KeyboardInput { input, is_synthetic, .. } => {
                            if !is_synthetic {
                                context.key_input(input)
                            }
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            context.mouse_input(button, state)
                        }
                        WindowEvent::ModifiersChanged(modifiers) => {
                            context.key_modifier_change(modifiers)
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            context.cursor_move((position.x, position.y));
                        }
                        _ => return,
                    }

                Event::MainEventsCleared => {
                    if rendering {
                        context.render();
                    } else {
                        context.update();
                    }
                }
                _ => (),
            }
        });
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        if size.0 > 0 && size.1 > 0 {
            self.size = PhysicalSize::new(size.0, size.1);
            self.config.width = size.0;
            self.config.height = size.1;
            self.surface.configure(&self.device, &self.config);
        }
    }
}