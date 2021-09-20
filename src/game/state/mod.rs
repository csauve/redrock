pub mod player_control;
pub mod physics_state;
pub mod object_state;
pub mod camera_state;

mod prelude {
    #[macro_export]
    macro_rules! state {
        ($i:item) => {
            #[derive(Copy, Clone, Default)]
            #[repr(C)]
            $i
        };
    }
    
    #[macro_export]
    macro_rules! state_nodef {
        ($i:item) => {
            #[derive(Copy, Clone, Default)]
            #[repr(C)]
            $i
        };
    }

    pub use state;
    pub use state_nodef;
    pub use crate::util::saltybuffer::{SaltyBuffer, SaltyId, NONE};
    pub use crate::game::tags::TagId;
}

use prelude::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub tick: u32,
    // Expressed in Earth Gs
    pub gravity: f32,
    pub player_control: player_control::PlayerControl,
    pub camera: camera_state::CameraState,
    pub objects: SaltyBuffer<object_state::ObjectState, 1024>,
    pub physics: SaltyBuffer<physics_state::PhysicsState, 1024>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tick: 0,
            gravity: 1.0,
            player_control: player_control::PlayerControl::default(),
            camera: camera_state::CameraState::default(),
            objects: SaltyBuffer::<object_state::ObjectState, 1024>::new(),
            physics: SaltyBuffer::<physics_state::PhysicsState, 1024>::new(),
        }
    }
}
