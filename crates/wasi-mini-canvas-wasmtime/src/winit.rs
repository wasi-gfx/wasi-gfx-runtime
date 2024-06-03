use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use crate::{MiniCanvas, MiniCanvasDesc, MiniCanvasProxy};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wasi_graphics_context_wasmtime::DisplayApi;
use winit::{
    application::ApplicationHandler,
    dpi::Size,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::ModifiersState,
    window::{Window, WindowAttributes, WindowId},
};

pub fn create_wasi_winit_event_loop() -> (WasiWinitEventLoop, WasiWinitEventLoopProxy) {
    let event_loop = WasiWinitEventLoop {
        event_loop: winit::event_loop::EventLoop::<MainThreadAction>::with_user_event()
            .build()
            .unwrap(),
    };
    let message_sender = WasiWinitEventLoopProxy {
        proxy: event_loop.event_loop.create_proxy(),
    };
    (event_loop, message_sender)
}

pub struct WasiWinitEventLoop {
    event_loop: EventLoop<MainThreadAction>,
}

impl WasiWinitEventLoop {
    /// This has to be run on the main thread.
    /// This call will block the thread.
    pub fn run(self) {
        let proxies: Arc<Mutex<HashMap<WindowId, MiniCanvasProxy>>> = Default::default();

        {
            let proxies = Arc::clone(&proxies);
            thread::spawn(move || loop {
                for (_, proxy) in proxies.lock().unwrap().iter() {
                    proxy.animation_frame();
                }
                sleep(Duration::from_millis(16));
            });
        }

        struct MyWindow(pub Window);
        impl HasDisplayHandle for MyWindow {
            fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
                self.0.display_handle()
            }
        }
        impl HasWindowHandle for MyWindow {
            fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
                self.0.window_handle()
            }
        }
        impl DisplayApi for MyWindow {
            fn height(&self) -> u32 {
                self.0.inner_size().height
            }

            fn width(&self) -> u32 {
                self.0.inner_size().width
            }
        }

        #[derive(Default)]
        struct App {
            pointer_pos: HashMap<WindowId, (f64, f64)>,
            modifiers: HashMap<WindowId, ModifiersState>,
            proxies: HashMap<WindowId, MiniCanvasProxy>,
            arc_proxies: Arc<Mutex<HashMap<WindowId, MiniCanvasProxy>>>,
        }

        impl ApplicationHandler<MainThreadAction> for App {
            fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
                // TODO:
            }

            fn user_event(&mut self, event_loop: &ActiveEventLoop, event: MainThreadAction) {
                match event {
                    MainThreadAction::CreateWindow(desc, response_channel) => {
                        let window_options = WindowAttributes::default().with_inner_size(
                            Size::Logical((desc.width as f64, desc.height as f64).into()),
                        );
                        let window = event_loop.create_window(window_options).unwrap();
                        // TODO: remove when window is drooped.
                        self.pointer_pos.insert(window.id(), (0.0, 0.0));
                        let window_id = window.id();

                        let canvas = MiniCanvas::new(Box::new(MyWindow(window)));

                        self.proxies.insert(window_id, canvas.proxy());
                        self.arc_proxies
                            .lock()
                            .unwrap()
                            .insert(window_id, canvas.proxy());

                        response_channel.send(canvas).unwrap();
                    }
                }
            }

            fn window_event(
                &mut self,
                _event_loop: &ActiveEventLoop,
                window_id: WindowId,
                event: WindowEvent,
            ) {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        self.pointer_pos
                            .insert(window_id, (position.x, position.y))
                            .unwrap();
                        if let Some(proxy) = self.proxies.get(&window_id) {
                            proxy.pointer_move(crate::PointerEvent {
                                x: position.x,
                                y: position.y,
                            });
                        }
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        self.modifiers.insert(window_id, modifiers.state());
                    }
                    WindowEvent::KeyboardInput { event: input, .. } => {
                        let modifiers = self.modifiers.get(&window_id).unwrap();
                        let event = crate::KeyEvent {
                            code: match input.physical_key {
                                winit::keyboard::PhysicalKey::Code(code) => format!("{code:?}"),
                                winit::keyboard::PhysicalKey::Unidentified(_) => todo!(),
                            },
                            key: match input.logical_key {
                                winit::keyboard::Key::Character(char) => char.to_string(),
                                _ => todo!(),
                            },
                            alt_key: modifiers.alt_key(),
                            ctrl_key: modifiers.control_key(),
                            meta_key: modifiers.super_key(),
                            shift_key: modifiers.shift_key(),
                        };
                        if let Some(proxy) = self.proxies.get(&window_id) {
                            match input.state {
                                ElementState::Pressed => {
                                    proxy.key_down(event);
                                }
                                ElementState::Released => {
                                    proxy.key_up(event);
                                }
                            }
                        }
                    }
                    WindowEvent::MouseInput { state, .. } => {
                        let (pointer_x, pointer_y) = self.pointer_pos.get(&window_id).unwrap();
                        let event = crate::PointerEvent {
                            x: *pointer_x,
                            y: *pointer_y,
                        };
                        if let Some(proxy) = self.proxies.get(&window_id) {
                            match state {
                                ElementState::Pressed => {
                                    proxy.pointer_down(event);
                                }
                                ElementState::Released => {
                                    proxy.pointer_up(event);
                                }
                            }
                        }
                    }
                    WindowEvent::Resized(new_size) => {
                        if let Some(proxy) = self.proxies.get(&window_id) {
                            proxy.canvas_resize(crate::ResizeEvent {
                                height: new_size.height,
                                width: new_size.width,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut app = App::default();
        app.arc_proxies = Arc::clone(&proxies);
        self.event_loop.run_app(&mut app).unwrap();
    }
}

#[derive(Clone)]
pub struct WasiWinitEventLoopProxy {
    proxy: EventLoopProxy<MainThreadAction>,
}

impl WasiWinitEventLoopProxy {
    pub async fn create_window(&self, desc: MiniCanvasDesc) -> MiniCanvas {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreateWindow(desc, sender))
            .unwrap();
        receiver.await.unwrap()
    }
}

#[derive(Debug)]
enum MainThreadAction {
    CreateWindow(MiniCanvasDesc, oneshot::Sender<MiniCanvas>),
}
