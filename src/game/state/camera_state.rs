use super::prelude::*;
use crate::math::Mat4x4;

state_nodef! {
    pub struct CameraState {
        pub object_attachment: SaltyId,
        pub v_fov: f32,
        pub near_clip: f32,
        pub far_clip: f32,
    }
}

impl Default for CameraState {
    fn default() -> CameraState {
        CameraState {
            object_attachment: NONE,
            v_fov: 90.0,
            near_clip: 0.1,
            far_clip: 100.0
        }
    }
}

impl CameraState {
    pub fn to_projection_matrix(&self, width: u32, height: u32) -> Mat4x4 {
        Mat4x4::projection(self.v_fov, width, height, self.near_clip, self.far_clip)
    }
}