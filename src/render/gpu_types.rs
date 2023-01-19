use cgmath::{prelude::*, Matrix4, Vector3, Vector4};

//std140
#[derive(Copy, Clone, Default)]
#[repr(C, align(4))]
pub struct GpuFloat(pub f32);

impl From<f32> for GpuFloat {
  #[inline]
  fn from(v: f32) -> GpuFloat {
    GpuFloat(v)
  }
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct GpuVec3(pub Vector3<f32>);

impl From<[f32; 3]> for GpuVec3 {
  #[inline]
  fn from(v: [f32; 3]) -> GpuVec3 {
    GpuVec3(Vector3::new(v[0], v[1], v[2]))
  }
}
impl From<Vector3<f32>> for GpuVec3 {
  #[inline]
  fn from(v: Vector3<f32>) -> GpuVec3 {
    GpuVec3(v)
  }
}

impl Default for GpuVec3 {
  fn default() -> GpuVec3 {
    [0., 0., 0.].into()
  }
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct GpuVec4(pub Vector4<f32>);

impl From<[f32; 4]> for GpuVec4 {
  #[inline]
  fn from(v: [f32; 4]) -> GpuVec4 {
    GpuVec4(Vector4::new(v[0], v[1], v[2], v[3]))
  }
}

impl From<Vector4<f32>> for GpuVec4 {
  #[inline]
  fn from(v: Vector4<f32>) -> GpuVec4 {
    GpuVec4(v)
  }
}

impl Default for GpuVec4 {
  fn default() -> GpuVec4 {
    [0., 0., 0., 0.].into()
  }
}

#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub struct GpuMat4(pub Matrix4<f32>);

impl From<Matrix4<f32>> for GpuMat4 {
  #[inline]
  fn from(v: Matrix4<f32>) -> GpuMat4 {
    GpuMat4(v)
  }
}

impl Default for GpuMat4 {
  fn default() -> GpuMat4 {
    GpuMat4(Matrix4::zero())
  }
}