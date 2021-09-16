use serde::{Serialize, Deserialize};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};
// use std::cmp::{max, min};

#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    #[inline]
    pub fn from_slice(slice: &[f32; 3]) -> Vec3f {
        Vec3f::new(slice[0], slice[1], slice[2])
    }

    #[inline]
    pub fn zero() -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn unit_x() -> Vec3f {
        Vec3f::new(1.0, 0.0, 0.0)
    }

    #[inline]
    pub fn unit_y() -> Vec3f {
        Vec3f::new(0.0, 1.0, 0.0)
    }

    #[inline]
    pub fn unit_z() -> Vec3f {
        Vec3f::new(0.0, 0.0, 1.0)
    }

    #[inline]
    pub fn cross(self, other: Vec3f) -> Vec3f {
        Vec3f::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    pub fn dot(self, other: Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn add(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    #[inline]
    pub fn add_mut(&mut self, other: Vec3f) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    #[inline]
    pub fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    #[inline]
    pub fn sub_mut(&mut self, other: Vec3f) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    #[inline]
    pub fn negate(self) -> Vec3f {
        Vec3f::new(-self.x, -self.y, -self.z)
    }

    #[inline]
    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Vec3f {
        self / self.length()
    }
}

impl Add<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        self.add(rhs)
    }
}

impl AddAssign<Vec3f> for Vec3f {
    fn add_assign(&mut self, rhs: Vec3f) {
        self.add_mut(rhs);
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        self.sub(rhs)
    }
}

impl SubAssign<Vec3f> for Vec3f {
    fn sub_assign(&mut self, rhs: Vec3f) {
        self.sub_mut(rhs);
    }
}

impl Div<f32> for Vec3f {
    type Output = Vec3f;
    fn div(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Vec3f;
    fn mul(self, rhs: f32) -> Vec3f {
        Vec3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for Vec3f {
    type Output = Vec3f;
    fn neg(self) -> Vec3f {
        self.negate()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_vec_dot() {
        let a = Vec3f::new(1.0, 2.0, 3.0);
        let b = Vec3f::new(4.0, 5.0, 6.0);
        assert_eq!(32.0, a.dot(b));
        assert_eq!(32.0, b.dot(a));

        assert!(a.dot(-a) < 0.0);
        assert!(a.dot(a) > 0.0);
        assert_eq!(0.0, a.dot(a.cross(b)));
    }
}
