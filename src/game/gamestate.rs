use crate::util::SaltyBuffer;
use crate::math::Vec3f;

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct ObjectState {
    pub position: Vec3f,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GameState {
    pub tick: u32,
    pub objects: SaltyBuffer<ObjectState, 1024>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tick: 0,
            objects: SaltyBuffer::<ObjectState, 1024>::new(),
        }
    }
}
