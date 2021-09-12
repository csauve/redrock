use crate::math::Vec3f;

pub struct Sphere {
    center: Vec3f,
    radius: f32,
}

pub struct Pill {
    start: Sphere,
    end: Sphere,
}
