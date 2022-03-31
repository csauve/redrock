use cgmath::{prelude::*, Vector3, Quaternion, Matrix4};
use super::prelude::*;
use super::physics_state::PhysicsState;
use crate::game::tags::{Map, Object};

state_nodef! {
    pub struct ObjectState {
        pub tag: TagId,
        pub position: Vector3<f32>,
        pub rotation: Quaternion<f32>,
        pub physics_id: SaltyId,
    }
}

impl Default for ObjectState {
    fn default() -> Self {
        ObjectState {
            tag: TagId::default(),
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            physics_id: NONE,
        }
    }
}

impl ObjectState {
    pub fn init(game_state: &mut GameState, map: &Map, object_tag_id: &TagId, position: Vector3<f32>, rotation: Quaternion<f32>) -> SaltyId {
        if let Some(object_tag) = map.object.get(object_tag_id) {
            
            let physics_id = if let Some(physics_tag_id) = object_tag.physics {
                PhysicsState::init(game_state, map, &physics_tag_id)
            } else {
                NONE
            };

            let object_state = ObjectState {
                tag: object_tag_id.clone(),
                position,
                rotation,
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

    pub fn to_transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) * Matrix4::from(self.rotation)
    }
}
