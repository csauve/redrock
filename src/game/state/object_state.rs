use crate::math::{Vec3f, Quaternion, Euler};
use super::prelude::*;
use super::super::tags::Object;

state! {
    pub struct ObjectState {
        pub tag: TagId,
        pub position: Vec3f,
        pub orientation: Euler,
        // pub rotation: Quaternion,
        pub physics: SaltyId,
    }
}
