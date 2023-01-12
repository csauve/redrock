pub mod tags;
pub mod actions;
pub mod state;

use state::game_state::GameState;
use tags::{Map, Placement};
use actions::PlayerAction;

pub struct Game {
    pub map: Map,
    pub state: GameState,
}

impl Game {
    pub fn load_map(map_path: &str) -> Game {
        let map = Map::load(map_path);
        let mut game = Game {
            state: GameState::init(&map),
            map,
        };
        game
    }

    pub fn apply_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Left(held) => {
                self.state.player_control.left = held;
            },
            PlayerAction::Right(held) => {
                self.state.player_control.right = held;
            },
            PlayerAction::Forward(held) => {
                self.state.player_control.forward = held;
            },
            PlayerAction::Back(held) => {
                self.state.player_control.back = held;
            },
            PlayerAction::Jump(held) => {
                self.state.player_control.up = held;
            },
            PlayerAction::Crouch(held) => {
                self.state.player_control.down = held;
            },
            PlayerAction::AimDelta(d_yaw, d_pitch) => {
                self.state.player_control.aim_delta(d_yaw, d_pitch);
            },
            _ => ()
        }
    }

    pub fn update(&mut self) {
        self.state.update(&self.map);
    }
}