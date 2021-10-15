use super::prelude::*;
use crate::math::Vec3f;

tag! {
    pub struct SceneryPlacement {
        pub position: Vec3f,
        pub object_type: TagId,
    }
}

tag! {
    pub struct Scenario {
        pub player_location: Vec3f,
        pub scenery: Option<Vec<SceneryPlacement>>,
    }
}
