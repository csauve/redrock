use std::collections::HashMap;
// use gltf::json::texture;
use wgpu;
use cgmath::{prelude::*, Matrix4, Vector3, Vector4};
use crate::game::Game;
use crate::game::state::{transform::Transform, object_state::ObjectState};
use crate::render::Window;

use super::common::{Texture, create_texture};
use super::model_pass::ModelPass;
use super::post_pass::PostPass;
use super::gpu_types::*;

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    model_pass: ModelPass,
    model_pass_output: Texture,
    post_pass: PostPass,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let backend = wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::PRIMARY);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(&window.window) };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
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
        device.on_uncaptured_error(|err| {
            dbg!(err);
        });

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0], //Bgra8UnormSrgb
            width: window.window.inner_size().width,
            height: window.window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let model_pass = ModelPass::new(&device, &config);
        dbg!(config.format);
        let model_pass_output = create_texture(
            &device,
            config.width,
            config.height,
            config.format,
            wgpu::AddressMode::ClampToEdge,
            None
        );
        let post_pass = PostPass::new(&device, &model_pass_output, &config);

        Renderer {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            model_pass,
            model_pass_output,
            post_pass,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = std::cmp::max(1, width);
        self.config.height = std::cmp::max(1, height);
        self.surface.configure(&self.device, &self.config);
        // let model_pass_output = create_texture(
        //     &self.device,
        //     self.config.width,
        //     self.config.height,
        //     self.config.format,
        //     wgpu::AddressMode::ClampToEdge,
        //     None
        // );
        self.model_pass.resize(&self.device, &self.config);
        // self.post_pass.resize //TODO
    }

    pub fn render(&mut self, game: &Game) {        
        if let Ok(output) = self.surface.get_current_texture() {
            let surface_output_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

            self.model_pass.render(game, &self.model_pass_output.view, &mut self.queue, &self.config, &self.device);
            self.post_pass.render(&self.device, &self.model_pass_output.view, &surface_output_view, &mut self.queue);

            output.present();
        }
    }
}

