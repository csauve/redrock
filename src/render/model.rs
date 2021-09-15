use crate::math::Vec3f;
use std::vec::Vec;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Vertex {
    position: Vec3f,
    colour: Vec3f,
}

impl Vertex {
    pub fn at(pos: &[f32; 3]) -> Vertex {
        Vertex {position: Vec3f::from_slice(pos), colour: Vec3f::default()}
    }
}

pub type FaceIndices = [u16; 3];

#[derive(Copy, Clone, PartialEq, Debug, Default)]
#[repr(C)]
pub struct Instance {
    pub position: Vec3f,
}

#[derive(Clone, Debug)]
pub struct Model {
    vertices: Vec<Vertex>,
    indices: Vec<FaceIndices>,
    shader: String,
}

impl Model {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<FaceIndices>, shader: String) -> Model {
        Model {
            vertices,
            indices,
            shader,
        }
    }

    pub fn vertices_slice<'a>(&'a self) -> &'a[Vertex] {
        &self.vertices
    }

    pub fn indices_slice<'a>(&'a self) -> &'a[FaceIndices] {
        &self.indices
    }

    pub fn shader_name<'a>(&'a self) -> &'a str {
        &self.shader
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_mesh_size() {
        assert_eq!(24, std::mem::size_of::<Vertex>());
        assert_eq!(6, std::mem::size_of::<FaceIndices>());
    }
}