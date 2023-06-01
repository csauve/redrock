use cgmath::Vector3;

use super::collider::CollideWith;
use super::sphere::Sphere;

pub struct FixedPlane {
  pub normal: Vector3<f32>,
  pub d: f32,
}

impl CollideWith<Sphere> for FixedPlane {
  fn toc(&self, other: &Sphere) -> Option<f32> {
      other.toc(self)
  }
}