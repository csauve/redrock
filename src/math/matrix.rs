#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mat4x4([[f32; 4]; 4]);

impl Mat4x4 {
    pub fn projection(v_fov: f32, width: u32, height: u32, near: f32, far: f32) -> Mat4x4 {
        let aspect = width as f32 / height as f32;
        let v_2_tan = (v_fov / 2.0).tan();
        let range = near - far;
        Mat4x4([
            [1.0 / (aspect * v_2_tan), 0.0, 0.0, 0.0],
            [0.0, 1.0 / v_2_tan, 0.0, 0.0],
            [0.0, 0.0, (-near - far) / range, (2.0 * far * near) / range],
            [0.0, 0.0, 1.0, 0.0],
        ])
    }

    pub fn transpose(&self) -> Mat4x4 {
        Mat4x4([
            [self.0[0][0], self.0[1][0], self.0[2][0], self.0[3][0]],
            [self.0[0][1], self.0[1][1], self.0[2][1], self.0[3][1]],
            [self.0[0][2], self.0[1][2], self.0[2][2], self.0[3][2]],
            [self.0[0][3], self.0[1][3], self.0[2][3], self.0[3][3]],
        ])
    }

    pub fn to_slice(&self) -> &[[f32; 4]; 4] {
        &self.0
    }
}