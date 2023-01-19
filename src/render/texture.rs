use wgpu::{self, Extent3d};
use std::io::{BufReader};
use std::fs::File;
use image::{self, GenericImageView};
// use gltf::json::texture;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn load(path: &str, device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Option<Texture> {
        if let Ok(f) = File::open(path) {
            let mut reader = BufReader::new(f);
            if let Ok(img) = image::load(reader, image::ImageFormat::Tiff) {
                let rgba8 = img.to_rgba8();
                let dimensions = img.dimensions();
                let texture = Texture::create(
                    device,
                    dimensions.0,
                    dimensions.1,
                    format,
                    wgpu::AddressMode::Repeat,
                    None
                );
                queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &texture.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &rgba8,
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                        rows_per_image: std::num::NonZeroU32::new(dimensions.1),
                    },
                    Extent3d {
                        width: dimensions.0,
                        height: dimensions.1,
                        depth_or_array_layers: 1,
                    },
                );
                return Some(texture);
            }
        }
        None
    }

    pub fn create(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        address_mode: wgpu::AddressMode,
        compare: Option<wgpu::CompareFunction>,
    ) -> Texture {
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
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };
        let texture = device.create_texture(&descriptor);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: address_mode,
            address_mode_v: address_mode,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });
        Texture {
            width,
            height,
            format,
            texture,
            view,
            sampler,
        }
}
}