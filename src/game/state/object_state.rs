use crate::math::Vec3f;
use super::prelude::*;
use super::super::tags::Object;

state! {
    pub struct ObjectState {
        pub position: Vec3f,
        pub physics: SaltyId,
    }
}
