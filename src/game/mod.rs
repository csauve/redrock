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

    pub fn update(&mut self, actions: &[PlayerAction]) -> bool {
        self.state.update(&self.map, actions)
    }
}