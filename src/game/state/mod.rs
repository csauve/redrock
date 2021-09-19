pub mod player_control;
pub mod physics_state;
pub mod object_state;

mod prelude {
    #[macro_export]
    macro_rules! state {
        ($i:item) => {
            #[derive(Copy, Clone, Default)]
            #[repr(C)]
            $i
        };
    }

    pub use state;
    pub use crate::util::saltybuffer::{SaltyBuffer, SaltyId, NONE};
    pub use crate::game::tags::TagId;
}

use prelude::*;
use object_state::ObjectState;
use physics_state::PhysicsState;
use player_control::PlayerControl;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub tick: u32,
    // Expressed in Earth Gs
    pub gravity: f32,
    pub player_control: PlayerControl,
    pub objects: SaltyBuffer<ObjectState, 1024>,
    pub physics: SaltyBuffer<PhysicsState, 1024>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tick: 0,
            gravity: 1.0,
            player_control: PlayerControl::default(),
            objects: SaltyBuffer::<ObjectState, 1024>::new(),
            physics: SaltyBuffer::<PhysicsState, 1024>::new(),
        }
    }
}
