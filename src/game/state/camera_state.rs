use super::prelude::*;

state! {
    pub struct CameraState {
        pub object_attachment: SaltyId,
        pub fov: f32,
    }
}