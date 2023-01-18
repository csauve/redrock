use wgpu;
use wgpu::util::{DeviceExt, BufferInitDescriptor};

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