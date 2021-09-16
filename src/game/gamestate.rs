use crate::util::{SaltyBuffer, SaltyId, NONE};
use crate::tags::Map;
use crate::math::Vec3f;

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct ObjectState {
    pub position: Vec3f,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PhysicsConstants {
    // Expressed in Earth Gs
    pub gravity: f32,
}

impl Default for PhysicsConstants {
    fn default() -> PhysicsConstants {
        PhysicsConstants {
            gravity: 1.0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub tick: u32,
    pub physics_constants: PhysicsConstants,
    pub player_object_id: SaltyId,
    pub objects: SaltyBuffer<ObjectState, 1024>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tick: 0,
            physics_constants: PhysicsConstants::default(),
            player_object_id: NONE,
            objects: SaltyBuffer::<ObjectState, 1024>::new(),
        }
    }

    pub fn from_map(map: &Map) -> GameState {
        let mut state = GameState::new();
        state.physics_constants.gravity = map.globals.gravity_scale;
        state.player_object_id = state.objects.add(ObjectState {
            position: map.scenario.player_location,
        }).unwrap();
        state
    }
}
