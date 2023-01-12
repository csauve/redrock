use std::borrow::Cow;
use std::collections::HashMap;
use gltf::json::texture;
use wgpu;
use wgpu::util::{DeviceExt, BufferInitDescriptor};
use cgmath::{prelude::*, Matrix4, Vector3};
use crate::game::Game;
use crate::render::Window;

use super::model::{Vertex, FaceIndices, ModelInstance, Model};

const MAX_INSTANCES: usize = 128;

struct ModelBuffers {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: wgpu::Buffer,
    face_count: u32,
}

struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    zbuffer: Texture,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    model_pipeline: wgpu::RenderPipeline,
    models: HashMap<String, ModelBuffers>,
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
                features: wgpu::Features::empty(),
                // features: wgpu::Features::NON_FILL_POLYGON_MODE,
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
                //normal
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
                //transform
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 5,
                },
                //colour
                wgpu::VertexAttribute {
                    offset: 64,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 6,
                },
            ]
        };

        let model_instances_buffer = Renderer::create_buffer(
            &device,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            &[ModelInstance::default(); MAX_INSTANCES]
        );

        let camera_buffer = Renderer::create_buffer(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[Matrix4::<f32>::zero()]
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
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        });

        let zbuffer = Renderer::create_zbuffer_texture(&device, config.width, config.height);

        Renderer {
            instance,
            surface,
            zbuffer,
            adapter,
            device,
            queue,
            config,

            camera_buffer,
            camera_bind_group,

            model_pipeline,
            models: HashMap::new(),
            model_instances_buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = std::cmp::max(1, width);
        self.config.height = std::cmp::max(1, height);
        self.surface.configure(&self.device, &self.config);
        self.zbuffer = Renderer::create_zbuffer_texture(&self.device, self.config.width, self.config.height);
    }

    fn create_zbuffer_texture(device: &wgpu::Device, width: u32, height: u32) -> Texture {
        let descriptor = wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = device.create_texture(&descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });
        Texture {
            texture,
            view,
            sampler,
        }
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

    pub fn load_model(&mut self, path: &str) {
        if !self.models.contains_key(path) {
            let model = Model::from_gltf(path).expect("Failed to load model");
            let vertex_buffer = Renderer::create_buffer(&self.device, wgpu::BufferUsages::VERTEX, model.vertices_slice());
            let index_buffer = Renderer::create_buffer(&self.device, wgpu::BufferUsages::INDEX, model.indices_slice());
            self.models.insert(path.into(), ModelBuffers {
                vertex_buffer,
                vertex_count: model.vertices_slice().len() as u32,
                index_buffer,
                face_count: model.indices_slice().len() as u32,
            });
        }
    }

    pub fn render(&mut self, game: &Game) {
        if let Ok(wgpu::SurfaceFrame {output, ..}) = self.surface.get_current_frame() {
            //load camera buffer
            let mut camera_matrix = game.state.camera.to_camera_matrix(self.config.width, self.config.height);
            self.queue.write_buffer(&self.camera_buffer, 0, Renderer::bytes_slice(&[camera_matrix]));
            
            //load model buffers
            let mut model_instances: HashMap<String, Vec<ModelInstance>> = HashMap::new();
            for (_id, object_state) in game.state.objects.iter() {
                if let Some(object_tag) = game.map.object.get(&object_state.tag) {
                    let instance = ModelInstance {
                        transform: object_state.to_transform_matrix(),
                        colour: Vector3::new(object_tag.colour[0], object_tag.colour[1], object_tag.colour[2]),
                    };
                    let model_path: String = object_tag.model.into();
                    if !model_instances.contains_key(&model_path) {
                        model_instances.insert(model_path.clone(), Vec::new());
                        self.load_model(&model_path);
                    }
                    model_instances.get_mut(&model_path).unwrap().push(instance);
                }
            }

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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.zbuffer.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            //render models
            let mut instances_total: usize = 0;
            render_pass.set_pipeline(&self.model_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            for (model_path, instances) in model_instances.iter() {
                let start_index = instances_total;
                let instances_remaining = MAX_INSTANCES - instances_total;
                let instances_added = std::cmp::min(instances_remaining, instances.len());
                if instances_remaining == 0 {
                    break;
                }
                instances_total += instances_added;
                self.queue.write_buffer(
                    &self.model_instances_buffer,
                    start_index as u64 * std::mem::size_of::<ModelInstance>() as u64,
                    unsafe {
                        std::slice::from_raw_parts(instances.as_ptr() as *const u8, instances_added * std::mem::size_of::<ModelInstance>())
                    }
                );
                let instance_range = (start_index as u32)..(start_index as u32 + instances_added as u32);
                if let Some(model) = self.models.get(model_path) {
                    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.set_vertex_buffer(1, self.model_instances_buffer.slice(..));
                    render_pass.draw_indexed(0..model.face_count, 0, instance_range);
                }
            }

            drop(render_pass);
            self.queue.submit(std::iter::once(encoder.finish()));
        }
    }
}

