use super::prelude::*;
use cgmath::{prelude::*, Matrix4, Vector3, Point3, Quaternion, PerspectiveFov, Rad};

state_nodef! {
    pub struct CameraState {
        pub object_attachment: SaltyId,
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
        pub v_fov: f32,
        pub near_clip: f32,
        pub far_clip: f32,
    }
}

impl Default for CameraState {
    fn default() -> CameraState {
        CameraState {
            object_attachment: NONE,
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::zero(),
            v_fov: 90.0,
            near_clip: 0.1,
            far_clip: 100.0,
        }
    }
}

impl CameraState {
    pub fn to_camera_matrix(&self, width: u32, height: u32) -> Matrix4<f32> {
        let proj: Matrix4<f32> = PerspectiveFov::<f32> {
            fovy: Rad::<f32>(self.v_fov),
            aspect: width as f32 / height as f32,
            near: self.near_clip,
            far: self.far_clip,
        }.into();

        let trans = Matrix4::from_translation(-self.position);
        // let space: Matrix4<f32> = Matrix4::look_to_rh(Point3::new(0.0, 0.0, 0.0), Vector3::unit_x(), Vector3::unit_z());
        let mut space: Matrix4<f32> = Matrix4::zero();
        space[1][0] = -1.0; //x becomes +y
        space[2][1] = 1.0; //y becomes +z
        space[0][2] = -1.0; //z becomes -x
        space[3][3] = 1.0;
        let rot: Matrix4<f32> = self.rotation.into();
        proj
            * space
            * rot.transpose()
            * trans
    }
}