mod vec;

// pub use vec::*;
// pub use matrix::*;
// pub use euler::*;

pub const TAU: f32 = std::f32::consts::TAU;
pub const PI: f32 = std::f32::consts::PI;
pub const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;
const RADIANS_PER_DEGREE: f32 = TAU / 360.0;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * RADIANS_PER_DEGREE
}

#[inline]
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians / RADIANS_PER_DEGREE
}
