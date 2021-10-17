use cgmath::{prelude::*, Vector3, Quaternion, Matrix4};
use super::prelude::*;
use super::super::tags::Object;

state_nodef! {
    pub struct ObjectState {
        pub tag: TagId,
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
        pub physics: SaltyId,
    }
}

impl Default for ObjectState {
    fn default() -> Self {
        ObjectState {
            tag: TagId::default(),
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            physics: NONE,
        }
    }
}

impl ObjectState {
    pub fn to_transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)
    }
}