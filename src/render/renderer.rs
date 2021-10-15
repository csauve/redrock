use std::borrow::Cow;
use std::collections::HashMap;
use wgpu;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use crate::game::Game;
use crate::math::Mat4x4;
use crate::render::Window;

use super::model::{Vertex, FaceIndices, ModelInstance, Model};

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

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    model_pipeline: wgpu::RenderPipeline,
    model_buffers: HashMap<String, ModelBuffers>,
    model_instances_buffer: wgpu::Buffer,
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
                features: wgpu::Features::NON_FILL_POLYGON_MODE, // ::empty(),
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
            source: wgpu::ShaderSource::Wgsl(include_str!("model_shader.wgsl").into()),
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
            array_stride: std::mem::size_of::<ModelInstance>() as wgpu::BufferAddress,
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

        let model_instances_buffer = Renderer::create_buffer(
            &device,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            &[ModelInstance::default(); MAX_INSTANCES as usize]
        );

        let camera_buffer = Renderer::create_buffer(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            Mat4x4::default().to_slice()
        );

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                }
            ],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &&camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ]
        });
        
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor  {
            label: None,
            bind_group_layouts: &[
                &camera_bind_group_layout
            ],
            push_constant_ranges: &[],
        });

        let model_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                polygon_mode: wgpu::PolygonMode::Line,
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

            camera_buffer,
            camera_bind_group,

            model_pipeline,
            model_buffers: HashMap::new(),
            model_instances_buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = std::cmp::max(1, width);
        self.config.height = std::cmp::max(1, height);
        self.surface.configure(&self.device, &self.config);
    }

    fn create_buffer<T>(device: &wgpu::Device, usage: wgpu::BufferUsages, contents: &[T]) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            usage,
            contents: Renderer::bytes_slice(contents)
        })
    }

    fn bytes_slice<T>(data: &[T]) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<T>())
        }
    }

    pub fn render(&mut self, game: &Game) {
        //todo: move to resource cache
        if !self.model_buffers.contains_key("test") {
            let model = Model::from_gltf("maps/cube.gltf").expect("Failed to load model");
            let vertex_buffer = Renderer::create_buffer(&self.device, wgpu::BufferUsages::VERTEX, model.vertices_slice());
            let index_buffer = Renderer::create_buffer(&self.device, wgpu::BufferUsages::INDEX, model.indices_slice());
            self.model_buffers.insert("test".into(), ModelBuffers {
                vertex_buffer,
                vertex_count: model.vertices_slice().len() as u32,
                index_buffer,
                face_count: model.indices_slice().len() as u32,
            });
        }


        if let Ok(wgpu::SurfaceFrame {output, ..}) = self.surface.get_current_frame() {
            let mut camera_matrix = game.state.camera.to_camera_matrix(self.config.width, self.config.height);

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

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
            
            //render models
            render_pass.set_pipeline(&self.model_pipeline);
            let model_instances: Vec<ModelInstance> = game.state.objects.iter()
                .map(|(_id, object)| ModelInstance {
                    position: object.position,
                })
                .collect();
            self.queue.write_buffer(&self.model_instances_buffer, 0 as wgpu::BufferAddress, unsafe {
                std::slice::from_raw_parts(model_instances.as_ptr() as *const u8, std::cmp::min(MAX_INSTANCES as usize, model_instances.len()) * std::mem::size_of::<ModelInstance>())
            });
            self.queue.write_buffer(&self.camera_buffer, 0, Renderer::bytes_slice(camera_matrix.to_slice()));
            let model_buffer = self.model_buffers.get("test").unwrap();
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, model_buffer.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model_buffer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, self.model_instances_buffer.slice(..));
            render_pass.draw_indexed(0..model_buffer.face_count, 0, 0..(model_instances.len() as u32));
            
            drop(render_pass);
            self.queue.submit(std::iter::once(encoder.finish()));
        }
    }
}

