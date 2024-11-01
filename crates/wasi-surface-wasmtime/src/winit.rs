use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
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
    dpi::{PhysicalSize, Size},
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

            fn request_set_size(&self, width: Option<u32>, height: Option<u32>) {
                let _ = self.0.request_inner_size(PhysicalSize::new(
                    width.unwrap_or(self.width()),
                    height.unwrap_or(self.height()),
                ));
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
                        let mut window_options = WindowAttributes::default();
                        if let (Some(width), Some(height)) = (desc.width, desc.height) {
                            window_options = window_options.with_inner_size(Size::Logical(
                                (width as f64, height as f64).into(),
                            ));
                        }
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
                    MainThreadAction::Spawn(f, res) => {
                        res.send(f()).unwrap();
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
                            key: match input.physical_key {
                                winit::keyboard::PhysicalKey::Code(code) => code.try_into().ok(),
                                winit::keyboard::PhysicalKey::Unidentified(_) => None,
                            },
                            text: match input.logical_key {
                                winit::keyboard::Key::Character(char) => Some(char.to_string()),
                                winit::keyboard::Key::Named(_)
                                | winit::keyboard::Key::Unidentified(_)
                                | winit::keyboard::Key::Dead(_) => None,
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
                    WindowEvent::CloseRequested => std::process::exit(1),
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

    pub async fn spawn<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        let boxed = Box::new(|| {
            let res = f();
            Box::new(res) as Box<dyn Any + Send + Sync>
        });
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::Spawn(boxed, sender))
            .unwrap();
        *receiver.await.unwrap().downcast().unwrap()
    }
}

enum MainThreadAction {
    CreateWindow(MiniCanvasDesc, oneshot::Sender<MiniCanvas>),
    Spawn(
        Box<dyn FnOnce() -> Box<dyn Any + Send + Sync> + Send + Sync>,
        oneshot::Sender<Box<dyn Any + Send + Sync>>,
    ),
}

impl Debug for MainThreadAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateWindow(arg0, arg1) => f
                .debug_tuple("CreateWindow")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::Spawn(_, _) => f.debug_tuple("Spawn").finish(),
        }
    }
}

impl TryFrom<winit::keyboard::KeyCode> for crate::wasi::webgpu::surface::Key {
    type Error = ();

    fn try_from(value: winit::keyboard::KeyCode) -> Result<Self, Self::Error> {
        use crate::wasi::webgpu::surface::Key;
        match value {
            winit::keyboard::KeyCode::Backquote => Ok(Key::Backquote),
            winit::keyboard::KeyCode::Backslash => Ok(Key::Backslash),
            winit::keyboard::KeyCode::BracketLeft => Ok(Key::BracketLeft),
            winit::keyboard::KeyCode::BracketRight => Ok(Key::BracketRight),
            winit::keyboard::KeyCode::Comma => Ok(Key::Comma),
            winit::keyboard::KeyCode::Digit0 => Ok(Key::Digit0),
            winit::keyboard::KeyCode::Digit1 => Ok(Key::Digit1),
            winit::keyboard::KeyCode::Digit2 => Ok(Key::Digit2),
            winit::keyboard::KeyCode::Digit3 => Ok(Key::Digit3),
            winit::keyboard::KeyCode::Digit4 => Ok(Key::Digit4),
            winit::keyboard::KeyCode::Digit5 => Ok(Key::Digit5),
            winit::keyboard::KeyCode::Digit6 => Ok(Key::Digit6),
            winit::keyboard::KeyCode::Digit7 => Ok(Key::Digit7),
            winit::keyboard::KeyCode::Digit8 => Ok(Key::Digit8),
            winit::keyboard::KeyCode::Digit9 => Ok(Key::Digit9),
            winit::keyboard::KeyCode::Equal => Ok(Key::Equal),
            winit::keyboard::KeyCode::IntlBackslash => Ok(Key::IntlBackslash),
            winit::keyboard::KeyCode::IntlRo => Ok(Key::IntlRo),
            winit::keyboard::KeyCode::IntlYen => Ok(Key::IntlYen),
            winit::keyboard::KeyCode::KeyA => Ok(Key::KeyA),
            winit::keyboard::KeyCode::KeyB => Ok(Key::KeyB),
            winit::keyboard::KeyCode::KeyC => Ok(Key::KeyC),
            winit::keyboard::KeyCode::KeyD => Ok(Key::KeyD),
            winit::keyboard::KeyCode::KeyE => Ok(Key::KeyE),
            winit::keyboard::KeyCode::KeyF => Ok(Key::KeyF),
            winit::keyboard::KeyCode::KeyG => Ok(Key::KeyG),
            winit::keyboard::KeyCode::KeyH => Ok(Key::KeyH),
            winit::keyboard::KeyCode::KeyI => Ok(Key::KeyI),
            winit::keyboard::KeyCode::KeyJ => Ok(Key::KeyJ),
            winit::keyboard::KeyCode::KeyK => Ok(Key::KeyK),
            winit::keyboard::KeyCode::KeyL => Ok(Key::KeyL),
            winit::keyboard::KeyCode::KeyM => Ok(Key::KeyM),
            winit::keyboard::KeyCode::KeyN => Ok(Key::KeyN),
            winit::keyboard::KeyCode::KeyO => Ok(Key::KeyO),
            winit::keyboard::KeyCode::KeyP => Ok(Key::KeyP),
            winit::keyboard::KeyCode::KeyQ => Ok(Key::KeyQ),
            winit::keyboard::KeyCode::KeyR => Ok(Key::KeyR),
            winit::keyboard::KeyCode::KeyS => Ok(Key::KeyS),
            winit::keyboard::KeyCode::KeyT => Ok(Key::KeyT),
            winit::keyboard::KeyCode::KeyU => Ok(Key::KeyU),
            winit::keyboard::KeyCode::KeyV => Ok(Key::KeyV),
            winit::keyboard::KeyCode::KeyW => Ok(Key::KeyW),
            winit::keyboard::KeyCode::KeyX => Ok(Key::KeyX),
            winit::keyboard::KeyCode::KeyY => Ok(Key::KeyY),
            winit::keyboard::KeyCode::KeyZ => Ok(Key::KeyZ),
            winit::keyboard::KeyCode::Minus => Ok(Key::Minus),
            winit::keyboard::KeyCode::Period => Ok(Key::Period),
            winit::keyboard::KeyCode::Quote => Ok(Key::Quote),
            winit::keyboard::KeyCode::Semicolon => Ok(Key::Semicolon),
            winit::keyboard::KeyCode::Slash => Ok(Key::Slash),
            winit::keyboard::KeyCode::AltLeft => Ok(Key::AltLeft),
            winit::keyboard::KeyCode::AltRight => Ok(Key::AltRight),
            winit::keyboard::KeyCode::Backspace => Ok(Key::Backspace),
            winit::keyboard::KeyCode::CapsLock => Ok(Key::CapsLock),
            winit::keyboard::KeyCode::ContextMenu => Ok(Key::ContextMenu),
            winit::keyboard::KeyCode::ControlLeft => Ok(Key::ControlLeft),
            winit::keyboard::KeyCode::ControlRight => Ok(Key::ControlRight),
            winit::keyboard::KeyCode::Enter => Ok(Key::Enter),
            winit::keyboard::KeyCode::SuperLeft => Ok(Key::MetaLeft),
            winit::keyboard::KeyCode::SuperRight => Ok(Key::MetaRight),
            winit::keyboard::KeyCode::ShiftLeft => Ok(Key::ShiftLeft),
            winit::keyboard::KeyCode::ShiftRight => Ok(Key::ShiftRight),
            winit::keyboard::KeyCode::Space => Ok(Key::Space),
            winit::keyboard::KeyCode::Tab => Ok(Key::Tab),
            winit::keyboard::KeyCode::Convert => Ok(Key::Convert),
            winit::keyboard::KeyCode::KanaMode => Ok(Key::KanaMode),
            winit::keyboard::KeyCode::Lang1 => Ok(Key::Lang1),
            winit::keyboard::KeyCode::Lang2 => Ok(Key::Lang2),
            winit::keyboard::KeyCode::Lang3 => Ok(Key::Lang3),
            winit::keyboard::KeyCode::Lang4 => Ok(Key::Lang4),
            winit::keyboard::KeyCode::Lang5 => Ok(Key::Lang5),
            winit::keyboard::KeyCode::NonConvert => Ok(Key::NonConvert),
            winit::keyboard::KeyCode::Delete => Ok(Key::Delete),
            winit::keyboard::KeyCode::End => Ok(Key::End),
            winit::keyboard::KeyCode::Help => Ok(Key::Help),
            winit::keyboard::KeyCode::Home => Ok(Key::Home),
            winit::keyboard::KeyCode::Insert => Ok(Key::Insert),
            winit::keyboard::KeyCode::PageDown => Ok(Key::PageDown),
            winit::keyboard::KeyCode::PageUp => Ok(Key::PageUp),
            winit::keyboard::KeyCode::ArrowDown => Ok(Key::ArrowDown),
            winit::keyboard::KeyCode::ArrowLeft => Ok(Key::ArrowLeft),
            winit::keyboard::KeyCode::ArrowRight => Ok(Key::ArrowRight),
            winit::keyboard::KeyCode::ArrowUp => Ok(Key::ArrowUp),
            winit::keyboard::KeyCode::NumLock => Ok(Key::NumLock),
            winit::keyboard::KeyCode::Numpad0 => Ok(Key::Numpad0),
            winit::keyboard::KeyCode::Numpad1 => Ok(Key::Numpad1),
            winit::keyboard::KeyCode::Numpad2 => Ok(Key::Numpad2),
            winit::keyboard::KeyCode::Numpad3 => Ok(Key::Numpad3),
            winit::keyboard::KeyCode::Numpad4 => Ok(Key::Numpad4),
            winit::keyboard::KeyCode::Numpad5 => Ok(Key::Numpad5),
            winit::keyboard::KeyCode::Numpad6 => Ok(Key::Numpad6),
            winit::keyboard::KeyCode::Numpad7 => Ok(Key::Numpad7),
            winit::keyboard::KeyCode::Numpad8 => Ok(Key::Numpad8),
            winit::keyboard::KeyCode::Numpad9 => Ok(Key::Numpad9),
            winit::keyboard::KeyCode::NumpadAdd => Ok(Key::NumpadAdd),
            winit::keyboard::KeyCode::NumpadBackspace => Ok(Key::NumpadBackspace),
            winit::keyboard::KeyCode::NumpadClear => Ok(Key::NumpadClear),
            winit::keyboard::KeyCode::NumpadClearEntry => Ok(Key::NumpadClearEntry),
            winit::keyboard::KeyCode::NumpadComma => Ok(Key::NumpadComma),
            winit::keyboard::KeyCode::NumpadDecimal => Ok(Key::NumpadDecimal),
            winit::keyboard::KeyCode::NumpadDivide => Ok(Key::NumpadDivide),
            winit::keyboard::KeyCode::NumpadEnter => Ok(Key::NumpadEnter),
            winit::keyboard::KeyCode::NumpadEqual => Ok(Key::NumpadEqual),
            winit::keyboard::KeyCode::NumpadHash => Ok(Key::NumpadHash),
            winit::keyboard::KeyCode::NumpadMemoryAdd => Ok(Key::NumpadMemoryAdd),
            winit::keyboard::KeyCode::NumpadMemoryClear => Ok(Key::NumpadMemoryClear),
            winit::keyboard::KeyCode::NumpadMemoryRecall => Ok(Key::NumpadMemoryRecall),
            winit::keyboard::KeyCode::NumpadMemoryStore => Ok(Key::NumpadMemoryStore),
            winit::keyboard::KeyCode::NumpadMemorySubtract => Ok(Key::NumpadMemorySubtract),
            winit::keyboard::KeyCode::NumpadMultiply => Ok(Key::NumpadMultiply),
            winit::keyboard::KeyCode::NumpadParenLeft => Ok(Key::NumpadParenLeft),
            winit::keyboard::KeyCode::NumpadParenRight => Ok(Key::NumpadParenRight),
            winit::keyboard::KeyCode::NumpadStar => Ok(Key::NumpadStar),
            winit::keyboard::KeyCode::NumpadSubtract => Ok(Key::NumpadSubtract),
            winit::keyboard::KeyCode::Escape => Ok(Key::Escape),
            winit::keyboard::KeyCode::Fn => Ok(Key::Fn),
            winit::keyboard::KeyCode::FnLock => Ok(Key::FnLock),
            winit::keyboard::KeyCode::PrintScreen => Ok(Key::PrintScreen),
            winit::keyboard::KeyCode::ScrollLock => Ok(Key::ScrollLock),
            winit::keyboard::KeyCode::Pause => Ok(Key::Pause),
            winit::keyboard::KeyCode::BrowserBack => Ok(Key::BrowserBack),
            winit::keyboard::KeyCode::BrowserFavorites => Ok(Key::BrowserFavorites),
            winit::keyboard::KeyCode::BrowserForward => Ok(Key::BrowserForward),
            winit::keyboard::KeyCode::BrowserHome => Ok(Key::BrowserHome),
            winit::keyboard::KeyCode::BrowserRefresh => Ok(Key::BrowserRefresh),
            winit::keyboard::KeyCode::BrowserSearch => Ok(Key::BrowserSearch),
            winit::keyboard::KeyCode::BrowserStop => Ok(Key::BrowserStop),
            winit::keyboard::KeyCode::Eject => Ok(Key::Eject),
            winit::keyboard::KeyCode::LaunchApp1 => Ok(Key::LaunchApp1),
            winit::keyboard::KeyCode::LaunchApp2 => Ok(Key::LaunchApp2),
            winit::keyboard::KeyCode::LaunchMail => Ok(Key::LaunchMail),
            winit::keyboard::KeyCode::MediaPlayPause => Ok(Key::MediaPlayPause),
            winit::keyboard::KeyCode::MediaSelect => Ok(Key::MediaSelect),
            winit::keyboard::KeyCode::MediaStop => Ok(Key::MediaStop),
            winit::keyboard::KeyCode::MediaTrackNext => Ok(Key::MediaTrackNext),
            winit::keyboard::KeyCode::MediaTrackPrevious => Ok(Key::MediaTrackPrevious),
            winit::keyboard::KeyCode::Power => Ok(Key::Power),
            winit::keyboard::KeyCode::Sleep => Ok(Key::Sleep),
            winit::keyboard::KeyCode::AudioVolumeDown => Ok(Key::AudioVolumeDown),
            winit::keyboard::KeyCode::AudioVolumeMute => Ok(Key::AudioVolumeMute),
            winit::keyboard::KeyCode::AudioVolumeUp => Ok(Key::AudioVolumeUp),
            winit::keyboard::KeyCode::WakeUp => Ok(Key::WakeUp),
            winit::keyboard::KeyCode::Meta => Ok(Key::Super),
            winit::keyboard::KeyCode::Hyper => Ok(Key::Hyper),
            winit::keyboard::KeyCode::Turbo => Ok(Key::Turbo),
            winit::keyboard::KeyCode::Abort => Ok(Key::Abort),
            winit::keyboard::KeyCode::Resume => Ok(Key::Resume),
            winit::keyboard::KeyCode::Suspend => Ok(Key::Suspend),
            winit::keyboard::KeyCode::Again => Ok(Key::Again),
            winit::keyboard::KeyCode::Copy => Ok(Key::Copy),
            winit::keyboard::KeyCode::Cut => Ok(Key::Cut),
            winit::keyboard::KeyCode::Find => Ok(Key::Find),
            winit::keyboard::KeyCode::Open => Ok(Key::Open),
            winit::keyboard::KeyCode::Paste => Ok(Key::Paste),
            winit::keyboard::KeyCode::Props => Ok(Key::Props),
            winit::keyboard::KeyCode::Select => Ok(Key::Select),
            winit::keyboard::KeyCode::Undo => Ok(Key::Undo),
            winit::keyboard::KeyCode::Hiragana => Ok(Key::Hiragana),
            winit::keyboard::KeyCode::Katakana => Ok(Key::Katakana),
            winit::keyboard::KeyCode::F1 => Ok(Key::F1),
            winit::keyboard::KeyCode::F2 => Ok(Key::F2),
            winit::keyboard::KeyCode::F3 => Ok(Key::F3),
            winit::keyboard::KeyCode::F4 => Ok(Key::F4),
            winit::keyboard::KeyCode::F5 => Ok(Key::F5),
            winit::keyboard::KeyCode::F6 => Ok(Key::F6),
            winit::keyboard::KeyCode::F7 => Ok(Key::F7),
            winit::keyboard::KeyCode::F8 => Ok(Key::F8),
            winit::keyboard::KeyCode::F9 => Ok(Key::F9),
            winit::keyboard::KeyCode::F10 => Ok(Key::F10),
            winit::keyboard::KeyCode::F11 => Ok(Key::F11),
            winit::keyboard::KeyCode::F12 => Ok(Key::F12),
            winit::keyboard::KeyCode::F13 => Err(()),
            winit::keyboard::KeyCode::F14 => Err(()),
            winit::keyboard::KeyCode::F15 => Err(()),
            winit::keyboard::KeyCode::F16 => Err(()),
            winit::keyboard::KeyCode::F17 => Err(()),
            winit::keyboard::KeyCode::F18 => Err(()),
            winit::keyboard::KeyCode::F19 => Err(()),
            winit::keyboard::KeyCode::F20 => Err(()),
            winit::keyboard::KeyCode::F21 => Err(()),
            winit::keyboard::KeyCode::F22 => Err(()),
            winit::keyboard::KeyCode::F23 => Err(()),
            winit::keyboard::KeyCode::F24 => Err(()),
            winit::keyboard::KeyCode::F25 => Err(()),
            winit::keyboard::KeyCode::F26 => Err(()),
            winit::keyboard::KeyCode::F27 => Err(()),
            winit::keyboard::KeyCode::F28 => Err(()),
            winit::keyboard::KeyCode::F29 => Err(()),
            winit::keyboard::KeyCode::F30 => Err(()),
            winit::keyboard::KeyCode::F31 => Err(()),
            winit::keyboard::KeyCode::F32 => Err(()),
            winit::keyboard::KeyCode::F33 => Err(()),
            winit::keyboard::KeyCode::F34 => Err(()),
            winit::keyboard::KeyCode::F35 => Err(()),
            _ => todo!(),
        }
    }
}
