use super::{degrees_to_radians, Vec3f, Euler};

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Mat4x4([[f32; 4]; 4]);

impl Mat4x4 {
    pub fn projection(v_fov: f32, width: u32, height: u32, near: f32, far: f32) -> Mat4x4 {
        let aspect = width as f32 / height as f32;
        let v_2_tan = (degrees_to_radians(v_fov) / 2.0).tan();
        let range = near - far;
        Mat4x4([
            [-1.0 / (aspect * v_2_tan), 0.0, 0.0, 0.0],
            [0.0, 1.0 / v_2_tan, 0.0, 0.0],
            [0.0, 0.0, (-near - far) / range, 1.0],
            [0.0, 0.0, (2.0 * far * near) / range, 0.0],
        ])
    }

    pub fn rotation_from_euler(euler: &Euler) -> Mat4x4 {
        let sy = euler.yaw.sin(); //h = y
        let cy = euler.yaw.sin();
        let sp = euler.pitch.sin(); //a = p
        let cp = euler.pitch.cos();
        let sr = euler.roll.sin(); //b = r
        let cr = euler.roll.sin();
        Mat4x4([
            [
                cy * cp,
                -cy * sp * cr + sy * sr,
                cy * sp * sr + sy * cr,
                0.0
            ],
            [
                sp,
                cp * cr,
                -cp * sr,
                0.0
            ],
            [
                -sy * cp,
                sy * sp * cr + cy * sr,
                -sy * sp * sr + cy * cr,
                0.0
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn view(from: Vec3f, dir: Vec3f, up: Vec3f) -> Mat4x4 {
        let z = dir.normalize_or_zero();
        let x = up.cross(z).normalize_or_zero();
        let y = z.cross(x);
        let tx = -x.dot(from);
        let ty = -y.dot(from);
        let tz = -z.dot(from);
        // let orientation = Mat4x4([
        //     [x.x, y.x, z.x, 0.0],
        //     [x.y, y.y, z.y, 0.0],
        //     [x.z, y.z, z.z, 0.0],
        //     [0.0, 0.0, 0.0, 1.0],
        // ]);
        // let translation = Mat4x4([
        //     [1.0, 0.0, 0.0, 0.0],
        //     [0.0, 1.0, 0.0, 0.0],
        //     [0.0, 0.0, 1.0, 0.0],
        //     [-from.x, -from.y, -from.z, 1.0],
        // ]);
        // orientation.mult(&translation)
        Mat4x4([
            [x.x, y.x, z.x, 0.0],
            [x.y, y.y, z.y, 0.0],
            [x.z, y.z, z.z, 0.0],
            [tx, ty, tz, 1.0],
        ])
    }

    pub fn row(&self, index: usize) -> [f32; 4] {
        [
            self.0[0][index],
            self.0[1][index],
            self.0[2][index],
            self.0[3][index],
        ]
    }

    pub fn col(&self, index: usize) -> [f32; 4] {
        self.0[index]
    }

    pub fn mult(&self, other: &Mat4x4) -> Mat4x4 {
        let mut mat = Mat4x4::default();
        for col in 0..4 {
            for row in 0..4 {
                mat.0[col][row] = Self::dot(&self.row(row), &other.col(col));
            }
        }
        mat
    }

    pub fn mult_vec3f(&self, vec: Vec3f) -> Vec3f {
        Vec3f::new(
            Vec3f::from_slice4(&self.row(0)).dot(vec),
            Vec3f::from_slice4(&self.row(1)).dot(vec),
            Vec3f::from_slice4(&self.row(2)).dot(vec),
        )
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

    fn dot(a: &[f32; 4], b: &[f32; 4]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
    }
}