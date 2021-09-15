use std::borrow::Cow;
use std::collections::HashMap;
use wgpu;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::game::{GameState, ObjectState};
use crate::render::Window;

use super::model::{Vertex, FaceIndices, Instance};

const MAX_INSTANCES: u32 = 128;

struct ModelBuffers {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: wgpu::Buffer,
    face_count: u32,
}

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    pipeline: wgpu::RenderPipeline,
    model_buffers: HashMap<String, ModelBuffers>,
    instance_buffer: wgpu::Buffer,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let backend = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(&window.window) };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        }).await.expect("Failed to find adapter");

        println!("Found adapter {}", adapter.get_info().name);
        // adapter.limits();
        // adapter.features();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(), //NON_FILL_POLYGON_MODE?
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None
        ).await.expect("Failed to request device");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window.window.inner_size().width,
            height: window.window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);


        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor  {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vert_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //position
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0,
                },
                //colour
                wgpu::VertexAttribute {
                    offset: 12 as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 1,
                },
            ]
        };

        let instance_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                //position
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 2,
                },
            ]
        };

        let instances_init = &[Instance::default(); MAX_INSTANCES as usize];
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            contents: unsafe {
                std::slice::from_raw_parts(instances_init.as_ptr() as *const u8, instances_init.len() * std::mem::size_of::<Instance>())
            },
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main",
                buffers: &[vert_buffer_layout, instance_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        });

        Renderer {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,

            pipeline,
            model_buffers: HashMap::new(),
            instance_buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = std::cmp::max(1, width);
        self.config.height = std::cmp::max(1, height);
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, game_state: &GameState) {
        //todo: move to resource cache
        if !self.model_buffers.contains_key("test") {
            let verts = &[
                Vertex::at(&[0.5, -0.5, 0.0]),
                Vertex::at(&[0.5, 0.5, 0.0]),
                Vertex::at(&[-0.5, 0.5, 0.0]),
                Vertex::at(&[-0.5, -0.5, 0.0]),
            ];
            let indices: &[u16] = &[
                0, 1, 3,
                1, 2, 3,
            ];
            let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: wgpu::BufferUsages::VERTEX,
                contents: unsafe {
                    std::slice::from_raw_parts(verts.as_ptr() as *const u8, verts.len() * std::mem::size_of::<Vertex>())
                },
            });
            let index_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: wgpu::BufferUsages::INDEX,
                contents: unsafe {
                    std::slice::from_raw_parts(indices.as_ptr() as *const u8, indices.len() * std::mem::size_of::<u16>())
                },
            });
            self.model_buffers.insert("test".into(), ModelBuffers {
                vertex_buffer,
                vertex_count: verts.len() as u32,
                index_buffer,
                face_count: indices.len() as u32,
            });
        }


        if let Ok(wgpu::SurfaceFrame {output, ..}) = self.surface.get_current_frame() {
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        }
                    }],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.pipeline);
                for (_id, object) in game_state.objects.iter() {
                    let model_buffer = self.model_buffers.get("test").unwrap();
                    render_pass.set_vertex_buffer(0, model_buffer.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(model_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                    let instances: &[Instance] = &[
                        Instance {position: object.position},
                    ];
                    self.queue.write_buffer(&self.instance_buffer, 0 as wgpu::BufferAddress, unsafe {
                        std::slice::from_raw_parts(instances.as_ptr() as *const u8, instances.len() * std::mem::size_of::<Instance>())
                    });
                    render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
                    
                    render_pass.draw_indexed(0..model_buffer.face_count, 0, 0..(instances.len() as u32));
                }
            }

            self.queue.submit(std::iter::once(encoder.finish()));
        }
    }
}

