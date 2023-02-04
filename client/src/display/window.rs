use std::ops::Deref;
use std::sync::{Arc, Mutex};
use instant::Instant;
use tokio::runtime::Runtime;
use wgpu::{Backends, Device, InstanceDescriptor, Queue, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use game::Game;
use game::resources::content_pack::ContentPack;
use crate::client::Client;
use crate::renderer::renderer::RENDERER;
use crate::settings::GameSettings;

pub struct GameWindow {
    pub settings: GameSettings,
    pub modifiers: u32,
    pub surface: Surface,
    pub device: Arc<Mutex<Device>>,
    pub queue: Arc<Queue>,
    pub inner: Window,
    pub size: (u32, u32),
    pub config: SurfaceConfiguration
}

impl GameWindow {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        //Make sure it's >0 or it may crash
        let size = PhysicalSize::new(size.width.max(1), size.height.max(1));

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(&window).unwrap() };
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

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: surface.get_capabilities(&adapter).formats,
        };
        surface.configure(&device, &config);

        return Self {
            settings: GameSettings::new(),
            modifiers: 0,
            surface,
            device: Arc::new(Mutex::new(device)),
            queue: Arc::new(queue),
            inner: window,
            config,
            size: (size.width, size.height),
        };
    }

    pub fn run(game: Game, content: Box<dyn ContentPack>, runtime: Runtime) {
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
        let window = runtime.block_on(GameWindow::new(window));
        RENDERER.lock().unwrap().init(window.device.clone(), window.queue.clone(), (window.config.width, window.config.height));
        let mut context = Client::new(window, game, content);
        let mut next_frame = context.rendering_time(Instant::now());
        event_loop.run(move |ev, _, control_flow| {
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
                Event::RedrawRequested(window_id) if id == window_id => {
                    if context.render() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::MainEventsCleared => {
                    //Figure out if we're updating the game or rendering the game
                    if next_frame < context.update_time() {
                        context.request_redraw();
                        next_frame = context.rendering_time(next_frame);
                        *control_flow = ControlFlow::WaitUntil(next_frame.min(context.update_time()));
                    } else {
                        runtime.block_on(context.update());
                        *control_flow = ControlFlow::WaitUntil(context.update_time().min(next_frame));
                    }
                }
                _ => (),
            }
        });
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        if size.0 > 0 && size.1 > 0 {
            self.size = size;
            self.config.width = size.0;
            self.config.height = size.1;
            self.surface.configure(&self.device.lock().unwrap().deref(), &self.config);
        }
    }
}