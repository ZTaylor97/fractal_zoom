use wgpu::{Device, VertexBufferLayout};

use super::uniforms;

pub struct ShaderBundle {
    pub _shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
}

impl ShaderBundle {
    pub fn new(
        device: &Device,
        surface_format: &wgpu::TextureFormat,
        buffer_layout: &VertexBufferLayout,
    ) -> Self {
        // let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/mandelbrot.wgsl"));
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/julia.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Fractal shader layout"),
            bind_group_layouts: &[&uniforms::UniformData::bind_group_layout(&device)],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Fractal shader render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[buffer_layout.clone()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: *surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            _shader: shader,
            pipeline,
        }
    }
}
