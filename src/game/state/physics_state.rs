use cgmath::{prelude::*, Vector3, Quaternion};
use super::prelude::*;

state_nodef! {
    pub struct PhysicsState {
        pub tag: TagId,
        pub velocity: Vector3<f32>,
    }
}

impl Default for PhysicsState {
    fn default() -> Self {
        PhysicsState {
            tag: TagId::default(),
            velocity: Vector3::zero(),
        }
    }
}

impl PhysicsState {
    pub fn init(game_state: &mut GameState, _map: &Map, physics_tag_id: &TagId) -> SaltyId {
        game_state.physics.add(PhysicsState {
            tag: physics_tag_id.clone(),
            velocity: Vector3::zero(),
        }).unwrap()
    }
}