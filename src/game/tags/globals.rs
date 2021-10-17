use super::prelude::*;

tag! {
    pub struct Globals {
        pub gravity_scale: f32,
        pub player_object: TagId,
        pub player_accel: f32,
        pub player_drag_scale: f32,
        pub v_fov: Option<f32>,
    }
}

impl Globals {
    pub fn v_fov_as_radians(&self) -> f32 {
        self.v_fov.unwrap_or(70.0).to_radians()
    }
}
