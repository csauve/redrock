use super::prelude::*;
use std::f32::consts::{FRAC_PI_2};
use cgmath::prelude::*;
use cgmath::{Euler, InnerSpace, Vector3, Rad, Quaternion};

state_nodef! {
    pub struct PlayerControl {
        pub target_object: SaltyId,
        pub forward: bool,
        pub back: bool,
        pub right: bool,
        pub left: bool,
        pub up: bool,
        pub down: bool,
        pub yaw: f32,
        pub pitch: f32,
        pub roll: f32,
    }
}

impl Default for PlayerControl {
    fn default() -> Self {
        PlayerControl {
            target_object: NONE,
            forward: false,
            back: false,
            right: false,
            left: false,
            up: false,
            down: false,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}

impl PlayerControl {
    pub fn get_movement_vector(&self) -> Vector3<f32> {
        let mut result = Vector3::zero();
        if self.forward {
            result += Vector3::unit_x();
        }
        if self.back {
            result -= Vector3::unit_x();
        }
        if self.left {
            result += Vector3::unit_y();
        }
        if self.right {
            result -= Vector3::unit_y();
        }
        if self.up {
            result += Vector3::unit_z();
        }
        if self.down {
            result -= Vector3::unit_z();
        }
        if result.magnitude2() == 0.0 {
            return result;
        }
        result.normalize()
    }

    pub fn aim_delta(&mut self, d_yaw: f32, d_pitch: f32) {
        self.yaw += d_yaw;
        self.pitch = (self.pitch + d_pitch).clamp(-FRAC_PI_2, FRAC_PI_2);
    }

    pub fn get_aim_rot(&self) -> Quaternion<f32> {
        let yaw_q: Quaternion<f32> = Quaternion::from_angle_z(Rad(-self.yaw));
        let pitch_q: Quaternion<f32> = Quaternion::from_angle_y(Rad(self.pitch));
        let roll_q: Quaternion<f32> = Quaternion::from_angle_x(Rad(self.roll));
        return yaw_q * pitch_q * roll_q;
    }
}