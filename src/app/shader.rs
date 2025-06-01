use wgpu::{Device, VertexBufferLayout};

use super::uniforms;

pub struct ShaderBundle {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
}

impl ShaderBundle {
    pub fn new(
        device: &Device,
        surface_format: &wgpu::TextureFormat,
        buffer_layout: &VertexBufferLayout,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/julia.wgsl"));
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/mandelbrot.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Fractal shader layout"),
            bind_group_layouts: &[&uniforms::Uniforms::bind_group_layout(&device)],
            push_constant_ranges: &[],
        });

        let uniforms = uniforms::Uniforms::new();
        let uniform_buffer = uniforms.create_buffer(&device);

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniforms::Uniforms::bind_group_layout(&device),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
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
            shader,
            pipeline,
            uniform_bind_group,
            uniforms,
            uniform_buffer,
        }
    }
}
