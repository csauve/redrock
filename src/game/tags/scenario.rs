use super::prelude::*;
use cgmath::{prelude::*, Vector3, Quaternion, Rad};

tag! {
    pub struct Placement {
        pub pos: [f32; 3],
        pub rot: Option<[f32; 3]>
    }
}

impl Placement {
    pub fn to_pos(&self) -> Vector3<f32> {
        Vector3::new(self.pos[0], self.pos[1], self.pos[2])
    }

    pub fn to_rot(&self) -> Quaternion<f32> {
        
        if let Some([yaw, pitch, roll]) = self.rot {
            let roll_q: Quaternion<f32> = Quaternion::from_angle_x(Rad(roll.to_radians()));
            let pitch_q: Quaternion<f32> = Quaternion::from_angle_y(Rad(-pitch.to_radians()));
            let yaw_q: Quaternion<f32> = Quaternion::from_angle_z(Rad(-yaw.to_radians()));
            return yaw_q * pitch_q * roll_q;
        }
        Quaternion::zero()
    }
}

tag! {
    pub struct SceneryPlacement {
        pub position: Placement,
        pub object_type: TagId,
    }
}

tag! {
    pub struct Scenario {
        pub player_location: Placement,
        pub scenery: Option<Vec<SceneryPlacement>>,
    }
}
