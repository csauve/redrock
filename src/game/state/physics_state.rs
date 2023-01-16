use cgmath::{prelude::*, Vector3, Quaternion};
use super::transform::Transform;
use super::prelude::*;

state_nodef! {
    pub struct PhysicsState {
        pub tag: TagId,
        pub prev_transform: Transform,
        pub velocity: Vector3<f32>,
        pub angular_velocity: Quaternion<f32>,
    }
}

impl Default for PhysicsState {
    fn default() -> Self {
        PhysicsState {
            tag: TagId::default(),
            prev_transform: Transform::default(),
            velocity: Vector3::zero(),
            angular_velocity: Quaternion::zero(),
        }
    }
}

impl PhysicsState {
    pub fn init(game_state: &mut GameState, _map: &Map, physics_tag_id: &TagId, transform: Transform) -> SaltyId {
        game_state.physics.add(PhysicsState {
            tag: physics_tag_id.clone(),
            prev_transform: transform,
            velocity: Vector3::zero(),
            angular_velocity: Quaternion::zero(),
        }).unwrap()
    }
}