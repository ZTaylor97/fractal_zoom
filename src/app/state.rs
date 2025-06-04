use std::sync::Arc;
use std::time::{Duration, Instant};

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;

use super::renderer::Renderer;
pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    renderer: Renderer,
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
/// Holds all wgpu state.
impl<'a> State<'a> {
    /// Create and initialise State objects from a winit window.
    pub async fn new(window: Arc<Window>) -> State<'a> {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                label: None,
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let renderer = Renderer::new(&device, &surface_format);
        let now = Instant::now();
        let start_time = now;
        let paused_time = Duration::ZERO;
        let last_frame_time = now;

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            renderer,
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

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn draw(&mut self) {
        if self.zooming {
            self.zoom += 0.005;
        } else {
            self.zoom = f32::max(self.zoom - 0.05, 1.0);
        }

        let now = Instant::now();

        if self.paused {
            self.paused_time += now - self.last_frame_time;
        }

        self.last_frame_time = now;

        let elapsed = now - self.start_time - self.paused_time;
        let time_secs = elapsed.as_secs_f32();

        self.renderer
            .update(&mut self.queue, time_secs, self.zoom, self.offset);

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Failed to acquire next swap chain texture: {e}");
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer.draw(&mut rpass);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
