use cgmath::{prelude::*, Vector3, Quaternion};
use super::prelude::*;

state_nodef! {
    pub struct PhysicsState {
        pub velocity: Vector3<f32>,
    }
}

impl Default for PhysicsState {
    fn default() -> Self {
        PhysicsState {
            velocity: Vector3::zero(),
        }
    }
}