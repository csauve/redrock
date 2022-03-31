pub mod game_state;
pub mod player_control;
pub mod physics_state;
pub mod object_state;
pub mod camera_state;

use crate::game::Placement;
use physics_state::PhysicsState;

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
            #[derive(Copy, Clone)]
            #[repr(C)]
            $i
        };
    }

    pub use state;
    pub use state_nodef;
    pub use crate::util::saltybuffer::{SaltyBuffer, SaltyId, NONE};
    pub use crate::game::tags::TagId;
    pub use crate::game::tags::Map;
    pub use super::game_state::GameState;
}
