pub mod tags;
pub mod actions;
pub mod state;

use crate::math::Vec3f;
use crate::util::saltybuffer::{SaltyId, NONE};
use state::{GameState, object_state::ObjectState, physics_state::PhysicsState};
use tags::{Map, TagId};
use actions::PlayerAction;

pub struct Game {
    pub map: Map,
    pub state: GameState,
}

impl Game {
    pub fn load_map(map_path: &str) -> Game {
        let mut game = Game {
            map: Map::load(map_path),
            state: GameState::new()
        };
        game.state.gravity = game.map.globals.gravity_scale;
        game.state.player_object_id = game.spawn_object(
            game.map.scenario.player_location,
            game.map.globals.player_object.clone(),
        ).unwrap();
        
        game
    }

    pub fn apply_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Left(held) => {
                if let Some(player_state) = self.state.objects.get(self.state.player_object_id) {
                    if let Some(physics_state) = self.state.physics.get_mut(player_state.physics) {
                        physics_state.velocity.x = if held { -0.01 } else { 0.0 };
                    }
                }
            },
            _ => ()
        }
    }

    pub fn spawn_object(&mut self, position: Vec3f, tag_id: TagId) -> Option<SaltyId> {
        if let Some(tag) = self.map.object.get(&tag_id) {
            let mut physics_sid: SaltyId = NONE;
            if tag.physics.is_some() {
                if let Some(phys_tag) = self.map.physics.get(tag.physics.as_ref().unwrap()) {
                    let phys_state = PhysicsState {
                        velocity: Vec3f::default(),
                    };
                    if let Some(sid) = self.state.physics.add(phys_state) {
                        physics_sid = sid;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            let object_state = ObjectState {
                position,
                physics: physics_sid,
            };
            //todo: cleanup if this fails
            return self.state.objects.add(object_state);
        }
        None
    }

    pub fn update(&mut self) {    

        for (_id, object_state) in self.state.objects.iter_mut() {
            if let Some(physics_state) = self.state.physics.get(object_state.physics) {
                object_state.position.x += physics_state.velocity.x;
            }
        }
    
        self.state.tick = self.state.tick.wrapping_add(1);
    }
}