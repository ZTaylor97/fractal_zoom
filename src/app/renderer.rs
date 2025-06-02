use std::time::Instant;

use wgpu::{Device, Queue, RenderPass, TextureFormat};

use super::{quad::Quad, shader::ShaderBundle};

pub struct Renderer {
    quad: Quad,
    shader_bundle: ShaderBundle,
    start_time: Instant,
}

impl Renderer {
    pub fn new(device: &Device, surface_format: &TextureFormat) -> Self {
        let quad = Quad::new(device);
        let shader_bundle = ShaderBundle::new(device, surface_format, &quad.vertex_buffer_layout);
        let start_time = Instant::now();

        Self {
            quad,
            shader_bundle,
            start_time,
        }
    }

    pub fn update(&mut self, queue: &mut Queue) {
        let elapsed = self.start_time.elapsed();
        let time_secs = elapsed.as_secs_f32();

        self.shader_bundle.uniforms.update(time_secs);
        self.shader_bundle
            .uniforms
            .write_to_gpu(queue, &self.shader_bundle.uniform_buffer);
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.shader_bundle.pipeline);
        render_pass.set_bind_group(0, &self.shader_bundle.uniform_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.quad.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.quad.index_count, 0, 0..1);
    }
}
