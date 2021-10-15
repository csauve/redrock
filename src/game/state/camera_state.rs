use super::prelude::*;
use crate::math::{Mat4x4, Vec3f, Euler};

state_nodef! {
    pub struct CameraState {
        pub object_attachment: SaltyId,
        pub position: Vec3f,
        pub v_fov: f32,
        pub near_clip: f32,
        pub far_clip: f32,
        pub aim: Euler,
    }
}

impl Default for CameraState {
    fn default() -> CameraState {
        CameraState {
            object_attachment: NONE,
            position: Vec3f::default(),
            v_fov: 90.0,
            near_clip: 0.1,
            far_clip: 100.0,
            aim: Euler::default(),
        }
    }
}

impl CameraState {
    pub fn to_camera_matrix(&self, width: u32, height: u32) -> Mat4x4 {
        let proj = Mat4x4::projection(self.v_fov, width, height, self.near_clip, self.far_clip);
        let view = Mat4x4::view(self.position, self.aim.to_vector(), Vec3f::unit_z());
        proj.mult(&view)
    }
}