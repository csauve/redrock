use std::ops::Deref;
use std::time::SystemTime;
use cgmath::{Euler, Matrix4, Matrix3, Quaternion, Rad, Vector3, prelude::*};

use super::prelude::*;
use super::camera_state::CameraState;
use super::player_control::PlayerControl;
use super::object_state::ObjectState;
use super::transform::Transform;
use super::PhysicsState;
use crate::game::PlayerAction;

const TICK_RATE: u32 = 120;
const TICK_DURATION_NANOS: u32 = 1000000000 / TICK_RATE;
const TICK_DURATION_SEC: f32 = 1.0 / TICK_RATE as f32;
// const MAX_TICKS_PER_FRAME: u32 = 10; //todo: prevent spiral of death

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub prev_time: SystemTime,
    pub accum_nanos: u128,
    pub tick: u32,
    // Expressed in Earth Gs
    pub gravity: f32,
    pub player_control: PlayerControl,
    pub camera: CameraState,
    pub objects: SaltyBuffer<ObjectState, 1024>,
    pub physics: SaltyBuffer<PhysicsState, 1024>,
}

impl GameState {
    pub fn init(map: &Map) -> GameState {
        let mut state = GameState {
            prev_time: SystemTime::now(),
            accum_nanos: 0,
            tick: 0,
            gravity: map.globals.gravity_scale,
            player_control: PlayerControl::default(),
            camera: CameraState::init(map),
            objects: SaltyBuffer::<ObjectState, 1024>::new(),
            physics: SaltyBuffer::<PhysicsState, 1024>::new(),
        };

        state.player_control.target_object = ObjectState::init(
            &mut state,
            map,
            &map.globals.player_object,
            map.scenario.player_location.to_transform()
        );
        state.camera.object_attachment = state.player_control.target_object;

        if let Some(ref scenery_vec) = map.scenario.scenery {
            for scenery in scenery_vec {
                ObjectState::init(
                    &mut state,
                    map,
                    &scenery.object_type,
                    scenery.position.to_transform()
                );
            }
        }

        state
    }

    pub fn apply_action(&mut self, action: &PlayerAction) {
        match *action {
            PlayerAction::Left(held) => {
                self.player_control.left = held;
            },
            PlayerAction::Right(held) => {
                self.player_control.right = held;
            },
            PlayerAction::Forward(held) => {
                self.player_control.forward = held;
            },
            PlayerAction::Back(held) => {
                self.player_control.back = held;
            },
            PlayerAction::Jump(held) => {
                self.player_control.up = held;
            },
            PlayerAction::Crouch(held) => {
                self.player_control.down = held;
            },
            PlayerAction::Boost(held) => {
                self.player_control.boost = held;
            },
            PlayerAction::AimDelta(d_yaw, d_pitch) => {
                self.player_control.aim_delta(d_yaw, d_pitch);
            },
            _ => ()
        }
    }

    pub fn update(&mut self, map: &Map, actions: &[PlayerAction]) -> bool {
        let curr_time = SystemTime::now();
        if let Ok(elapsed) = curr_time.duration_since(self.prev_time) {
            self.accum_nanos += elapsed.as_nanos();

            for action in actions {
                match action {
                    PlayerAction::Quit => {
                        return false;
                    },
                    action => {
                        self.apply_action(action);
                    },
                }
            }

            while self.accum_nanos >= TICK_DURATION_NANOS as u128 {
                self.update_fixed(map);
                self.accum_nanos -= TICK_DURATION_NANOS as u128;
            }

            self.update_variable(map);

            self.prev_time = curr_time;
        }
        true
    }

    pub fn update_variable(&mut self, map: &Map) {
        //player control physics
        if let Some(player_state) = self.objects.get_mut(self.player_control.target_object) {
            if let Some(physics_state) = self.physics.get_mut(player_state.physics_id) {
                player_state.transform.rotation = self.player_control.get_aim_rot();

                let mut movement_vec = self.player_control.get_movement_vector();
                movement_vec = player_state.transform.rotation.rotate_vector(movement_vec);

                physics_state.velocity += movement_vec * map.globals.player_accel * TICK_DURATION_SEC;

                let mut drag = map.globals.player_drag_scale * (
                    physics_state.velocity.magnitude() +
                    physics_state.velocity.magnitude2()
                );
                if self.player_control.boost {
                    drag *= 0.1;
                }
                if physics_state.velocity.magnitude2() != 0.0 {
                    physics_state.velocity -= physics_state.velocity.normalize_to(drag) * TICK_DURATION_SEC;
                }
            }
        }
    }

    pub fn update_fixed(&mut self, map: &Map) {
        //physics to position
        for (_id, object_state) in self.objects.iter_mut() {
            if let Some(object_tag) = map.get_object(&object_state.tag) {
                if let Some(physics_tag_id) = object_tag.physics {
                    if let Some(_physics_tag) = map.get_physics(&physics_tag_id) {
                        if let Some(physics_state) = self.physics.get_mut(object_state.physics_id) {
                            physics_state.prev_transform = object_state.transform;
                            object_state.transform.position += physics_state.velocity * TICK_DURATION_SEC;
                            object_state.transform.rotation += physics_state.angular_velocity * TICK_DURATION_SEC;
                        }
                    }
                }
            }
        }
    
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn get_tick_interpolation_fraction(&self) -> f32 {
        self.accum_nanos as f32 / TICK_DURATION_NANOS as f32
    }
}
