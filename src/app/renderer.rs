use wgpu::{Device, RenderPass, TextureFormat};

use super::{quad::Quad, shader::ShaderBundle};

pub struct Renderer {
    quad: Quad,
    shader_bundles: Vec<ShaderBundle>,
}

impl Renderer {
    pub fn new(device: &Device, surface_format: &TextureFormat) -> Self {
        let quad = Quad::new(device);
        let shader_bundles = vec![
            ShaderBundle::new(
                device,
                surface_format,
                &quad.vertex_buffer_layout,
                wgpu::include_wgsl!("../shaders/mandelbrot.wgsl"),
            ),
            ShaderBundle::new(
                device,
                surface_format,
                &quad.vertex_buffer_layout,
                wgpu::include_wgsl!("../shaders/julia.wgsl"),
            ),
        ];

        Self {
            quad,
            shader_bundles,
        }
    }

    pub fn draw(
        &self,
        render_pass: &mut RenderPass,
        uniform_bind_group: &wgpu::BindGroup,
        bundle_idx: usize,
    ) {
        // set shader
        render_pass.set_pipeline(&self.shader_bundles[bundle_idx].pipeline);
        // set buffers
        render_pass.set_bind_group(0, uniform_bind_group, &[]);

        // Draw the quad
        render_pass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.quad.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.quad.index_count, 0, 0..1);
    }
}
