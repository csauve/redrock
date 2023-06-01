use cgmath::Vector3;

use super::collider::CollideWith;
use super::fixed_plane::FixedPlane;

pub struct Sphere {
  pub radius: f32,
  pub position: Vector3<f32>,
  pub velocity: Vector3<f32>,
}

//todo: some things dont need continuous phys
impl CollideWith<FixedPlane> for Sphere {
  fn toc(&self, other: &FixedPlane) -> Option<f32> {
      
      None
  }
}