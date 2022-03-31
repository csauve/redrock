use super::prelude::*;
use super::camera_state::CameraState;
use super::player_control::PlayerControl;
use super::object_state::ObjectState;
use super::PhysicsState;

use cgmath::{Euler, Matrix4, Matrix3, Quaternion, Rad, Vector3, prelude::*};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
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
            map.scenario.player_location.to_pos(),
            map.scenario.player_location.to_rot()
        );
        state.camera.object_attachment = state.player_control.target_object;

        if let Some(ref scenery_vec) = map.scenario.scenery {
            for scenery in scenery_vec {
                ObjectState::init(
                    &mut state,
                    map,
                    &scenery.object_type,
                    scenery.position.to_pos(),
                    scenery.position.to_rot()
                );
            }
        }

        state
    }

    pub fn update(&mut self, map: &Map) {
        //player control physics
        if let Some(player_state) = self.objects.get_mut(self.player_control.target_object) {
            if let Some(physics_state) = self.physics.get_mut(player_state.physics_id) {
                player_state.rotation = self.player_control.get_aim_rot();

                let mut movement_vec = self.player_control.get_movement_vector();
                movement_vec = player_state.rotation.rotate_vector(movement_vec);
                // movement_vec = Quaternion::<f32>::from_angle_z(Rad(90.0f32.to_radians())).rotate_vector(movement_vec);

                physics_state.velocity += movement_vec * map.globals.player_accel;

                let drag = map.globals.player_drag_scale * (physics_state.velocity.magnitude() + physics_state.velocity.magnitude2());
                if physics_state.velocity.magnitude2() != 0.0 {
                    physics_state.velocity -= physics_state.velocity.normalize_to(drag);
                }
            }
        }

        //physics to position
        for (_id, object_state) in self.objects.iter_mut() {
            if let Some(object_tag) = map.get_object(&object_state.tag) {
                if let Some(physics_tag_id) = object_tag.physics {
                    if let Some(_physics_tag) = map.get_physics(&physics_tag_id) {
                        if let Some(physics_state) = self.physics.get(object_state.physics_id) {
                            object_state.position += physics_state.velocity;
                        }
                    }
                }
            }
        }

        let camera_attachment = self.camera.object_attachment;
        if camera_attachment.is_some() {
            if let Some(attached_obj) = self.objects.get(camera_attachment) {
                self.camera.position = attached_obj.position;
                self.camera.rotation = attached_obj.rotation;
            }
        }
    
        self.tick = self.tick.wrapping_add(1);
    }
}
