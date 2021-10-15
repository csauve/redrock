#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Quaternion {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
