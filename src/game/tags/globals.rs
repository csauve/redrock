use super::prelude::*;

tag! {
    pub struct Globals {
        pub gravity_scale: f32,
        pub player_object: TagId,
        pub player_accel: f32,
        pub player_drag_scale: f32,
    }
}
