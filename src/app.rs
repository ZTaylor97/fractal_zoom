use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{self, ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod quad;
mod renderer;
mod shader;
mod state;
mod uniforms;
mod vertex;

use state::State;

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit()
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(state) = self.state.as_mut() {
                    state.resize(physical_size);
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = self.state.as_mut() {
                    state.draw();
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(app_state) = self.state.as_mut() {
                    match (key, state) {
                        (KeyCode::Space, ElementState::Pressed) => {
                            app_state.paused = true;
                        }
                        (KeyCode::Space, ElementState::Released) => {
                            app_state.paused = false;
                        }
                        _ => (),
                    }
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => if let Some(app_state) = self.state.as_mut() {},
            _ => {}
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }
}
