use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
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
                            app_state.app_state.paused = true;
                        }
                        (KeyCode::Enter, ElementState::Pressed) => {
                            app_state.app_state.paused = false;
                        }
                        _ => (),
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(app_state) = self.state.as_mut() {
                    let size = self.window.as_ref().unwrap().inner_size();

                    if app_state.app_state.follow_mouse {
                        let old_pos = app_state.app_state.mouse_pos;
                        let new_pos = position;

                        let diff_x = ((new_pos.x - old_pos.x) / size.width as f64) as f32;
                        let diff_y = ((new_pos.y - old_pos.y) / size.height as f64) as f32;
                        app_state.app_state.offset[0] +=
                            diff_x * 2.0 * 1.5 / app_state.app_state.zoom;
                        app_state.app_state.offset[1] +=
                            diff_y * 2.0 * 1.5 / app_state.app_state.zoom;
                    }

                    // update mouse pos only after calculations are done
                    app_state.app_state.mouse_pos = position;
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(app_state) = self.state.as_mut() {
                    match button {
                        MouseButton::Left => match state {
                            ElementState::Pressed => {
                                app_state.app_state.mouse_click_point =
                                    app_state.app_state.mouse_pos;
                                app_state.app_state.follow_mouse = true;
                            }
                            ElementState::Released => {
                                app_state.app_state.follow_mouse = false;
                            }
                        },
                        MouseButton::Right => {
                            app_state.app_state.zooming = state == ElementState::Pressed;
                        }
                        MouseButton::Middle => todo!(),
                        MouseButton::Back => todo!(),
                        MouseButton::Forward => todo!(),
                        MouseButton::Other(_) => todo!(),
                    }
                }
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }
}
