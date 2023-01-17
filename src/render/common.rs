use wgpu;
use wgpu::util::{DeviceExt, BufferInitDescriptor};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
}

pub fn create_texture(
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
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
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

pub fn create_buffer<T>(device: &wgpu::Device, usage: wgpu::BufferUsages, contents: &[T]) -> wgpu::Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        usage,
        contents: bytes_slice(contents)
    })
}

pub fn bytes_slice<T>(data: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<T>())
    }
}