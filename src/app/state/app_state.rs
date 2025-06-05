use std::time::{Duration, Instant};

use winit::dpi::PhysicalPosition;

pub struct AppState {
    pub paused: bool,
    start_time: Instant,
    last_frame_time: Instant,
    paused_time: Duration,
    pub zoom: f32,
    pub zooming: bool,
    pub offset: [f32; 2],
    pub follow_mouse: bool,
    pub mouse_click_point: PhysicalPosition<f64>,
    pub mouse_pos: PhysicalPosition<f64>,
}

impl AppState {
    pub fn new() -> Self {
        let now = Instant::now();
        let start_time = now;
        let paused_time = Duration::ZERO;
        let last_frame_time = now;
        Self {
            paused: false,
            start_time,
            last_frame_time,
            paused_time,
            zoom: 1.0,
            zooming: false,
            offset: [0.0, 0.0],
            follow_mouse: false,
            mouse_click_point: PhysicalPosition { x: 0.0, y: 0.0 },
            mouse_pos: PhysicalPosition { x: 0.0, y: 0.0 },
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        if self.paused {
            self.paused_time += now - self.last_frame_time;
        }

        self.last_frame_time = now;
    }

    pub fn elapsed_time(&self) -> f32 {
        let now = Instant::now();

        let elapsed = now - self.start_time - self.paused_time;
        let time_secs = elapsed.as_secs_f32();
    }
}
