use super::prelude::*;
use crate::math::Vec3f;

state! {
    pub struct PlayerControl {
        pub target_object: SaltyId,
        pub forward: bool,
        pub back: bool,
        pub right: bool,
        pub left: bool,
    }
}

impl PlayerControl {
    pub fn get_vector(&self) -> Vec3f {
        let mut result = Vec3f::zero();
        if self.forward {
            result += Vec3f::unit_x();
        }
        if self.back {
            result -= Vec3f::unit_x();
        }
        if self.left {
            result += Vec3f::unit_y();
        }
        if self.right {
            result -= Vec3f::unit_y();
        }
        result.normalize_or_zero()
    }
}