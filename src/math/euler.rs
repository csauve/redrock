use super::{Mat4x4, HALF_PI, Vec3f};

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Euler {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Euler {
    pub fn to_matrix(&self) -> Mat4x4 {
        Mat4x4::rotation_from_euler(self)
    }

    pub fn add_delta(&mut self, d_yaw: f32, d_pitch: f32) {
        self.yaw = self.yaw + d_yaw;
        self.pitch = (self.pitch + d_pitch).clamp(-HALF_PI, HALF_PI);
    }

    pub fn to_vector(&self) -> Vec3f {
        Vec3f::new(
            f32::cos(self.yaw),
            -f32::sin(self.yaw),
            f32::sin(self.pitch)
        )
    }
}