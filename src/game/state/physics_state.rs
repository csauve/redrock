use crate::math::Vec3f;
use super::prelude::*;

state! {
    pub struct PhysicsState {
        pub velocity: Vec3f,
    }
}