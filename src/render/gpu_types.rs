use cgmath::{prelude::*, Matrix4, Vector3, Vector4};

//std140
#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct GpuFloat(pub f32);

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct GpuVec3(pub Vector3<f32>);

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct GpuVec4(pub Vector4<f32>);

#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub struct GpuMat4(pub Matrix4<f32>);