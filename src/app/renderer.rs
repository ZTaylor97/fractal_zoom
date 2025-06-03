use std::time::Instant;

use wgpu::{Device, Queue, RenderPass, TextureFormat};

use super::{quad::Quad, shader::ShaderBundle};

pub struct Renderer {
    quad: Quad,
    shader_bundle: ShaderBundle,
}

impl Renderer {
    pub fn new(device: &Device, surface_format: &TextureFormat) -> Self {
        let quad = Quad::new(device);
        let shader_bundle = ShaderBundle::new(device, surface_format, &quad.vertex_buffer_layout);

        Self {
            quad,
            shader_bundle,
        }
    }

    pub fn update(
        &mut self,
        queue: &mut Queue,
        new_time: f32,
        new_zoom: f32,
        new_offset: [f32; 2],
    ) {
        self.shader_bundle
            .uniforms
            .update(new_time, new_zoom, new_offset);
        self.shader_bundle
            .uniforms
            .write_to_gpu(queue, &self.shader_bundle.uniform_buffer);
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        // set shader
        render_pass.set_pipeline(&self.shader_bundle.pipeline);
        // set buffers
        render_pass.set_bind_group(0, &self.shader_bundle.uniform_bind_group, &[]);

        // Draw the quad
        render_pass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.quad.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.quad.index_count, 0, 0..1);
    }
}
