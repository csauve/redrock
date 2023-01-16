use cgmath::{prelude::*, Vector3, Quaternion, Matrix3, Matrix4};
use super::prelude::*;
use super::transform::Transform;
use super::physics_state::PhysicsState;
use crate::game::tags::{Map, Object};

state_nodef! {
    pub struct ObjectState {
        pub tag: TagId,
        pub transform: Transform,
        pub physics_id: SaltyId,
    }
}

impl Default for ObjectState {
    fn default() -> Self {
        ObjectState {
            tag: TagId::default(),
            transform: Transform::default(),
            physics_id: NONE,
        }
    }
}

impl ObjectState {
    pub fn init(game_state: &mut GameState, map: &Map, object_tag_id: &TagId, transform: Transform) -> SaltyId {
        if let Some(object_tag) = map.object.get(object_tag_id) {
            
            let physics_id = if let Some(physics_tag_id) = object_tag.physics {
                PhysicsState::init(game_state, map, &physics_tag_id, transform)
            } else {
                NONE
            };

            let object_state = ObjectState {
                tag: object_tag_id.clone(),
                transform,
                physics_id,
            };
            //todo: cleanup if this fails
            return game_state.objects.add(object_state).unwrap();
        }

        SaltyId::none()
    }

    pub fn cleanup(game_state: &mut GameState, object_id: SaltyId) {
        if let Some(object) = game_state.objects.remove(object_id) {
            game_state.physics.remove(object.physics_id);
        }
    }
}
