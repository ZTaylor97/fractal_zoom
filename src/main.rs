use winit::event_loop::{ControlFlow, EventLoop};

mod app;

use app::App;
fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Application error: {e}")
    }
}
