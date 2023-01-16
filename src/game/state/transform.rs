use cgmath::{prelude::*, Vector3, Quaternion, Matrix3, Matrix4};
use super::prelude::*;

state_nodef! {
    pub struct Transform {
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
        }
    }
}

impl Transform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)
    }

    pub fn to_rotation_matrix(&self) -> Matrix3<f32> {
        Matrix3::from(self.rotation)
    }

    pub fn interpolate(a: &Transform, b: &Transform, factor: f32) -> Transform {
        Transform {
            position: a.position * (1.0 - factor) + b.position * factor,
            rotation: a.rotation.lerp(b.rotation, factor),
        }
    }
}