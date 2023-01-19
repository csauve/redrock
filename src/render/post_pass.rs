use wgpu;
use cgmath::{prelude::*, Vector4};
use super::common::{create_buffer, bytes_slice};
use super::texture::Texture;
use super::gpu_types::*;

pub struct PostPass {
    vertices_buffer: wgpu::Buffer,
    indices_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    effects_buffer: wgpu::Buffer,
}

#[derive(Copy, Clone, Default)]
#[repr(C, align(16))]
struct EffectsUniform {
    multiply_colour: GpuVec4,
    screen_colour: GpuVec4,
    blur_radius: GpuFloat,
}

impl PostPass {
    pub fn new(device: &wgpu::Device, prev_pass_texture: &Texture, config: &wgpu::SurfaceConfiguration) -> PostPass {
        let shader = device.create_shader_module(wgpu::include_wgsl!("post_shader.wgsl"));

        let vertices_buffer = create_buffer(device, wgpu::BufferUsages::VERTEX, &[
            [0f32, 0f32],
            [1f32, 0f32],
            [1f32, 1f32],
            [0f32, 1f32],
        ]);

        let indices_buffer = create_buffer(device, wgpu::BufferUsages::INDEX, &[
            0u16,
            1u16,
            2u16,
            0u16,
            2u16,
            3u16,
        ]);

        let effects_buffer = create_buffer(
            device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[EffectsUniform::default()]
        );

        let vert_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //position
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 0,
                },
            ]
        };

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("post bind group layout"),
            entries: &[
                //prev pass texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None
                },
                //prev pass sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                },
                //effects
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("post bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&prev_pass_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&prev_pass_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: effects_buffer.as_entire_binding(),
                },
            ]
        });
        
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor  {
            label: Some("post pipeline layout"),
            bind_group_layouts: &[
                &bind_group_layout
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("post pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main",
                buffers: &[vert_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        PostPass {
            vertices_buffer,
            indices_buffer,
            bind_group,
            pipeline,
            effects_buffer,
        }
    }

    pub fn render(&self, device: &wgpu::Device, input_view: &wgpu::TextureView, output_view: &wgpu::TextureView, queue: &mut wgpu::Queue) {
        let effects_uniform = EffectsUniform {
            multiply_colour: Vector4::new(1.0, 0.0, 0.0, 0.0).into(),
            screen_colour: Vector4::new(1.0, 0.0, 0.0, 0.0).into(),
            blur_radius: GpuFloat(0.005),
        };
        queue.write_buffer(&self.effects_buffer, 0, bytes_slice(&[effects_uniform]));
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("post encoder"),
        });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("post pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                }
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertices_buffer.slice(..));
        render_pass.set_index_buffer(self.indices_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);

        drop(render_pass);
        queue.submit(std::iter::once(encoder.finish()));
    }
}