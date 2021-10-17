pub mod tags;
pub mod actions;
pub mod state;

use cgmath::{Euler, Matrix4, Matrix3, Quaternion, Rad, Vector3, prelude::*};
use crate::util::saltybuffer::{SaltyId, NONE};
use state::{GameState, object_state::ObjectState, physics_state::PhysicsState};
use tags::{Map, TagId, Placement};
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
            &game.map.scenario.player_location.clone(),
            game.map.globals.player_object.clone(),
        ).unwrap();
        game.state.camera.v_fov = game.map.globals.v_fov_as_radians();

        if let Some(ref scenery_vec) = game.map.scenario.scenery {
            for scenery in scenery_vec.clone() {
                game.spawn_object(&scenery.position, scenery.object_type.clone());
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
                self.state.player_control.aim_delta(d_yaw, d_pitch);
            },
            _ => ()
        }
    }

    pub fn spawn_object(&mut self, placement: &Placement, tag_id: TagId) -> Option<SaltyId> {
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
                position: placement.to_pos(),
                rotation: placement.to_rot(),
                physics: physics_sid,
            };
            //todo: cleanup if this fails
            return self.state.objects.add(object_state);
        }
        None
    }

    pub fn update(&mut self) {
        let globals = &self.map.globals;

        //player control physics
        if let Some(player_state) = self.state.objects.get_mut(self.state.player_control.target_object) {
            if let Some(physics_state) = self.state.physics.get_mut(player_state.physics) {
                player_state.rotation = self.state.player_control.get_aim_rot();

                let mut movement_vec = self.state.player_control.get_movement_vector();
                movement_vec = player_state.rotation.rotate_vector(movement_vec);
                // movement_vec = Quaternion::<f32>::from_angle_z(Rad(90.0f32.to_radians())).rotate_vector(movement_vec);

                physics_state.velocity += movement_vec * globals.player_accel;

                let drag = globals.player_drag_scale * (physics_state.velocity.magnitude() + physics_state.velocity.magnitude2());
                if physics_state.velocity.magnitude2() != 0.0 {
                    physics_state.velocity -= physics_state.velocity.normalize_to(drag);
                }
            }
        }

        //physics to position
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
                self.state.camera.rotation = attached_obj.rotation;
            }
        }
    
        self.state.tick = self.state.tick.wrapping_add(1);
    }
}