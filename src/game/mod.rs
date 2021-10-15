pub mod tags;
pub mod actions;
pub mod state;

use crate::math::{Euler, Vec3f};
use crate::util::saltybuffer::{SaltyId, NONE};
use state::{GameState, object_state::ObjectState, physics_state::PhysicsState};
use tags::{Map, TagId};
use actions::PlayerAction;

use self::state::player_control;

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
        game.state.player_control.target_object = game.spawn_object(
            game.map.scenario.player_location,
            game.map.globals.player_object.clone(),
        ).unwrap();

        if let Some(ref scenery_vec) = game.map.scenario.scenery {
            for scenery in scenery_vec.clone() {
                game.spawn_object(scenery.position, scenery.object_type.clone());
            }
        }

        game.state.camera.object_attachment = game.state.player_control.target_object;
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
                self.state.player_control.aim.add_delta(d_yaw, d_pitch);
            },
            _ => ()
        }
    }

    pub fn spawn_object(&mut self, position: Vec3f, tag_id: TagId) -> Option<SaltyId> {
        if let Some(tag) = self.map.object.get(&tag_id) {
            let mut physics_sid: SaltyId = NONE;
            if tag.physics.is_some() {
                if let Some(_phys_tag) = self.map.physics.get(tag.physics.as_ref().unwrap()) {
                    let phys_state = PhysicsState::default();
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
                tag: tag_id,
                position,
                physics: physics_sid,
                orientation: Euler::default(),
            };
            //todo: cleanup if this fails
            return self.state.objects.add(object_state);
        }
        None
    }

    pub fn update(&mut self) {
        let globals = &self.map.globals;

        if let Some(player_state) = self.state.objects.get_mut(self.state.player_control.target_object) {
            if let Some(physics_state) = self.state.physics.get_mut(player_state.physics) {
                let local_movement_vec = self.state.player_control.get_movement_vector();
                player_state.orientation = self.state.player_control.aim;
                let world_relative_movement_vec = player_state.orientation.to_matrix().mult_vec3f(local_movement_vec);
                physics_state.velocity += world_relative_movement_vec * globals.player_accel;
                let drag = globals.player_drag_scale * (physics_state.velocity.length() + physics_state.velocity.length().powi(2));
                physics_state.velocity -= physics_state.velocity.normalize_or_zero() * drag;
            }
        }

        for (_id, object_state) in self.state.objects.iter_mut() {
            if let Some(object_tag) = self.map.get_object(&object_state.tag) {
                if let Some(physics_tag_id) = object_tag.physics {
                    if let Some(_physics_tag) = self.map.get_physics(&physics_tag_id) {
                        if let Some(physics_state) = self.state.physics.get(object_state.physics) {
                            object_state.position += physics_state.velocity;
                        }
                    }
                }
            }
        }

        let camera_attachment = self.state.camera.object_attachment;
        if camera_attachment.is_some() {
            if let Some(attached_obj) = self.state.objects.get(camera_attachment) {
                self.state.camera.position = attached_obj.position;
                self.state.camera.aim = attached_obj.orientation;
            }
        }
    
        self.state.tick = self.state.tick.wrapping_add(1);
    }
}