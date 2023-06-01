mod sphere;
mod fixed_plane;
mod collider;

pub use sphere::Sphere;
pub use fixed_plane::FixedPlane;

mod test {
  use crate::math::realcmp;

use super::{FixedPlane, Sphere, collider::CollideWith};
  use cgmath::Vector3;

  #[test]
  fn test_collision_time() {
    let floor = FixedPlane {
      normal: Vector3::unit_z(),
      d: 10.,
    };
    let ball = Sphere {
      radius: 10.,
      position: Vector3::new(0., 0., 2.),
      velocity: Vector3::new(0., 0., -1.),
    };
    let toc = ball.toc(&floor).unwrap();
    assert!(realcmp(toc, 2.0, 0.001))
  }
}