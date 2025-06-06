use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

use app_state::AppState;

use crate::app::state::render_state::RenderState;
use crate::app::uniforms::Uniforms;

mod app_state;
mod render_state;
pub struct State<'a> {
    render_state: RenderState<'a>,
    pub app_state: AppState,
    uniforms: Uniforms,
}
/// Holds all wgpu state.
impl<'a> State<'a> {
    /// Create and initialise State objects from a winit window.
    pub async fn new(window: Arc<Window>) -> State<'a> {
        let render_state = RenderState::new(window).await;

        let app_state = AppState::new();

        let uniforms = Uniforms::new(&render_state.device);

        Self {
            render_state,
            app_state,
            uniforms,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.render_state.resize(new_size);
    }

    pub fn draw(&mut self) {
        self.app_state.update();

        if self.app_state.zooming {
            self.app_state.zoom += 0.005;
        } else {
            self.app_state.zoom = f32::max(self.app_state.zoom - 0.05, 1.0);
        }

        self.uniforms.update(
            &mut self.render_state.queue,
            self.app_state.elapsed_time(),
            self.app_state.zoom,
            self.app_state.offset,
        );

        self.render_state.draw(&self.uniforms.uniform_bind_group);
    }
}
