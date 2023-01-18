use cgmath::{prelude::*, Matrix4, Vector3, Vector4, Matrix3};
use std::collections::HashMap;
use wgpu;

use crate::game::Game;
use crate::game::state::{transform::Transform, object_state::ObjectState};

use super::common::{create_buffer, bytes_slice};
use super::texture::Texture;
use super::model::{Vertex, Model};
use super::gpu_types::*;

const MAX_INSTANCES: usize = 128;

struct LoadedModel {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: wgpu::Buffer,
    indices_count: u32,
}

struct LoadedTexture {
    texture: Texture,
    bind_group: wgpu::BindGroup,
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
struct CameraUniform {
    view_proj: GpuMat4,
    world_position: GpuVec3,
}

impl Default for CameraUniform {
    fn default() -> CameraUniform {
        CameraUniform {
            view_proj: GpuMat4(Matrix4::<f32>::one()),
            world_position: GpuVec3(Vector3::<f32>::zero()),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
struct EnvironmentUniform {
    fog_colour: GpuVec4,
    fog_min_distance: GpuFloat,
    fog_max_distance: GpuFloat,
    sun_colour: GpuVec3,
    sun_direction: GpuVec3,
}


#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ModelInstance {
    pub transform_matrix: Matrix4<f32>,
    pub normal_matrix: Matrix3<f32>,
    pub colour: Vector3<f32>,
    //todo: bone data
}

impl Default for ModelInstance {
    fn default() -> Self {
        ModelInstance {
            transform_matrix: Matrix4::one(),
            normal_matrix: Matrix3::one(),
            colour: Vector3::unit_x(),
        }
    }
}

impl Default for EnvironmentUniform {
    fn default() -> EnvironmentUniform {
        EnvironmentUniform {
            fog_colour: GpuVec4(Vector4::new(0.1, 0.1, 0.3, 0.8)),
            fog_min_distance: GpuFloat(1.0),
            fog_max_distance: GpuFloat(25.0),
            sun_colour: GpuVec3(Vector3::new(0.8, 0.8, 0.5)),
            sun_direction: GpuVec3(Vector3::new(0.1, 0.5, 1.0).normalize()),
        }
    }
}

pub struct ModelPass {
    models: HashMap<String, LoadedModel>,
    textures: HashMap<String, LoadedTexture>,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    camera_buffer: wgpu::Buffer,
    environment_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    model_instances_buffer: wgpu::Buffer,
    zbuffer: Texture,
}

impl ModelPass {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> ModelPass {
        let shader = device.create_shader_module(wgpu::include_wgsl!("model_shader.wgsl"));

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
                //tangent
                wgpu::VertexAttribute {
                    offset: 24 as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 2,
                },
                //bitangent
                wgpu::VertexAttribute {
                    offset: 36 as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 3,
                },
                //uv
                wgpu::VertexAttribute {
                    offset: 48 as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 4,
                },
            ]
        };

        let instance_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                //transform matrix
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 6,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    offset: 48,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 8,
                },
                //normal matrix
                wgpu::VertexAttribute {
                    offset: 64,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 9,
                },
                wgpu::VertexAttribute {
                    offset: 76,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 10,
                },
                wgpu::VertexAttribute {
                    offset: 88,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 11,
                },
                //colour
                wgpu::VertexAttribute {
                    offset: 100,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 12,
                },
            ]
        };

        let model_instances_buffer = create_buffer(
            device,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            &[ModelInstance::default(); MAX_INSTANCES]
        );

        let environment_buffer = create_buffer(
            device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[EnvironmentUniform::default()]
        );

        let camera_buffer = create_buffer(
            device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[CameraUniform::default()]
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("model bind group layout"),
            entries: &[
                //camera
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                },
                //environment
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                },
            ],
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("model textures bind group layout"),
            entries: &[
                //texture
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
                //sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("model bind group"),
            layout: &bind_group_layout,
            entries: &[
                //camera
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                //environment
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: environment_buffer.as_entire_binding(),
                },
            ]
        });
        
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor  {
            label: Some("model pipeline layout"),
            bind_group_layouts: &[
                &bind_group_layout,
                &texture_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("model pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vertex_main",
                buffers: &[vert_buffer_layout, instance_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fragment_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
            },
            multiview: None,
        });

        let zbuffer = Self::create_zbuffer_texture(device, config.width, config.height);

        ModelPass {
            models: HashMap::new(),
            textures: HashMap::new(),
            zbuffer,
            camera_buffer,
            environment_buffer,
            bind_group,
            pipeline,
            texture_bind_group_layout,
            model_instances_buffer,
        }
    }


    fn create_zbuffer_texture(device: &wgpu::Device, width: u32, height: u32) -> Texture {
        Texture::create(
            device,
            width,
            height,
            wgpu::TextureFormat::Depth32Float,
            wgpu::AddressMode::ClampToEdge,
            Some(wgpu::CompareFunction::LessEqual)
        )
    }

    pub fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.zbuffer = Self::create_zbuffer_texture(device, config.width, config.height);
    }

    fn load_texture(&mut self, path: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.textures.contains_key(path) {
            return;
        }
        if let Some(texture) = Texture::load(path, device, queue) {
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("model texture bind group"),
                layout: &self.texture_bind_group_layout,
                entries: &[
                    //texture
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    //sampler
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                ]
            });
            self.textures.insert(String::from(path), LoadedTexture {
                texture,
                bind_group
            });
        }
    }

    fn load_model(&mut self, path: &str, device: &wgpu::Device) {
        if !self.models.contains_key(path) {
            let model = Model::from_gltf(path).expect("Failed to load model");
            let vertex_buffer = create_buffer(device, wgpu::BufferUsages::VERTEX, model.vertices_slice());
            let index_buffer = create_buffer(device, wgpu::BufferUsages::INDEX, model.indices_slice());
            self.models.insert(path.into(), LoadedModel {
                vertex_buffer,
                vertex_count: model.vertices_slice().len() as u32,
                index_buffer,
                indices_count: model.indices_slice().len() as u32,
            });
        }
    }

  pub fn render(&mut self, game: &Game, output_view: &wgpu::TextureView, queue: &mut wgpu::Queue, config: &wgpu::SurfaceConfiguration, device: &wgpu::Device) {
    let interpolation_fraction = game.state.get_tick_interpolation_fraction();

    let diffuse_path = "maps/groundtile.tif";
    self.load_texture(diffuse_path, device, queue);
    let bump_path = "maps/groundtile_bump.tif";
    self.load_texture(bump_path, device, queue);

    //load camera buffer
    let camera_attachment = game.state.camera.object_attachment;
    let mut camera_transform = Transform::default();
    if camera_attachment.is_some() {
        if let Some(attached_obj) = game.state.objects.get(camera_attachment) {
            camera_transform = Self::interpolate_camera(game, attached_obj, interpolation_fraction);
        }
    }
    let camera_uniform = CameraUniform {
        view_proj: GpuMat4(game.state.camera.to_camera_matrix(config.width, config.height, &camera_transform)),
        world_position: GpuVec3(camera_transform.position),
    };
    queue.write_buffer(&self.camera_buffer, 0, bytes_slice(&[camera_uniform]));
    let environment_uniform = EnvironmentUniform::default();
    queue.write_buffer(&self.environment_buffer, 0, bytes_slice(&[environment_uniform]));

    //load model buffers
    let mut model_instances: HashMap<String, Vec<ModelInstance>> = HashMap::new();
    for (_id, object_state) in game.state.objects.iter() {
        if let Some(object_tag) = game.map.object.get(&object_state.tag) {
            let transform = Self::interpolate_object(game, object_state, interpolation_fraction);
            let instance = ModelInstance {
                transform_matrix: transform.to_matrix(),
                normal_matrix: transform.to_rotation_matrix(),
                colour: Vector3::new(object_tag.colour[0], object_tag.colour[1], object_tag.colour[2]),
            };
            let model_path: String = object_tag.model.into();
            if !model_instances.contains_key(&model_path) {
                model_instances.insert(model_path.clone(), Vec::new());
                self.load_model(&model_path, device);
            }
            model_instances.get_mut(&model_path).unwrap().push(instance);
        }
    }

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("model encoder"),
    });
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("model pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &output_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: environment_uniform.fog_colour.0.x as f64,
                    g: environment_uniform.fog_colour.0.y as f64,
                    b: environment_uniform.fog_colour.0.z as f64,
                    a: 1.0,
                }),
                store: true,
            }
        })],
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
    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &self.bind_group, &[]);
    for (model_path, instances) in model_instances.iter() {
        let start_index = instances_total;
        let instances_remaining = MAX_INSTANCES - instances_total;
        let instances_added = std::cmp::min(instances_remaining, instances.len());
        if instances_remaining == 0 {
            break;
        }
        instances_total += instances_added;
        queue.write_buffer(
            &self.model_instances_buffer,
            start_index as u64 * std::mem::size_of::<ModelInstance>() as u64,
            unsafe {
                std::slice::from_raw_parts(instances.as_ptr() as *const u8, instances_added * std::mem::size_of::<ModelInstance>())
            }
        );
        let instance_range = (start_index as u32)..(start_index as u32 + instances_added as u32);
        if let Some(model_bufs) = self.models.get(model_path) {
            let diffuse = self.textures.get(diffuse_path).unwrap();
            render_pass.set_bind_group(1, &diffuse.bind_group, &[]);
            let bump = self.textures.get(bump_path).unwrap();
            render_pass.set_bind_group(2, &bump.bind_group, &[]);

            render_pass.set_vertex_buffer(0, model_bufs.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model_bufs.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_vertex_buffer(1, self.model_instances_buffer.slice(..));
            render_pass.draw_indexed(0..model_bufs.indices_count, 0, instance_range);
        }
    }

    drop(render_pass);
    queue.submit(std::iter::once(encoder.finish()));
  }

  fn interpolate_object(game: &Game, object_state: &ObjectState, interpolation_fraction: f32) -> Transform {
    if let Some(phys) = game.state.physics.get(object_state.physics_id) {
        Transform::interpolate(&phys.prev_transform, &object_state.transform, interpolation_fraction)
    } else {
        object_state.transform
    }
}

  fn interpolate_camera(game: &Game, object_state: &ObjectState, interpolation_fraction: f32) -> Transform {
      let mut transform = Self::interpolate_object(game, object_state, interpolation_fraction);
      transform.rotation = object_state.transform.rotation;
      transform
  }
}