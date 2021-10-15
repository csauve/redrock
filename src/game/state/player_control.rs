use super::prelude::*;
use crate::math::{Euler, PI, Vec3f};

state! {
    pub struct PlayerControl {
        pub target_object: SaltyId,
        pub forward: bool,
        pub back: bool,
        pub right: bool,
        pub left: bool,
        pub up: bool,
        pub down: bool,
        pub aim: Euler,
    }
}

impl PlayerControl {
    pub fn get_movement_vector(&self) -> Vec3f {
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
        if self.up {
            result += Vec3f::unit_z();
        }
        if self.down {
            result -= Vec3f::unit_z();
        }
        result.normalize_or_zero()
    }

    pub fn get_aim_vector(&self) -> Vec3f {
        Vec3f::new(
            f32::cos(self.aim.yaw),
            -f32::sin(self.aim.yaw),
            f32::sin(self.aim.pitch)
        )
    }
}