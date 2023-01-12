use cgmath::{Matrix4, Vector3, prelude::*};
use std::vec::Vec;
use gltf;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    position: Vector3<f32>,
    normal: Vector3<f32>,
}

impl Vertex {
    pub fn new(pos: &[f32; 3], normal: &[f32; 3]) -> Vertex {
        Vertex {position: Vector3::new(pos[0], pos[1], pos[2]), normal: Vector3::new(normal[0], normal[1], normal[2])}
    }
}

//deprecated
pub type FaceIndices = [u16; 3];

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ModelInstance {
    pub transform: Matrix4<f32>,
    pub colour: Vector3<f32>,
    //todo: bone data
}

impl Default for ModelInstance {
    fn default() -> Self {
        ModelInstance {
            transform: Matrix4::zero(),
            colour: Vector3::unit_x(),
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Model {
    pub fn from_gltf(path: &str) -> Result<Model, String> {
        if let Ok((file, buffers, _images)) = gltf::import(path) {
            if let Some(scene) = file.default_scene() {
                if let Some(root_node) = scene.nodes().find(|n| n.name().map_or(false, |name| name == "root")) {
                    //todo: use the hierarchy -- for now we just use the root node's mesh
                    let mesh = root_node.mesh().unwrap();
                    for primitive in mesh.primitives() {
                        let primitive_reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                        let vertices: Vec<Vertex> = if let Some(vertices_reader) = primitive_reader.read_positions() {
                            if let Some(normals_reader) = primitive_reader.read_normals() {
                                normals_reader
                                    .zip(vertices_reader)
                                    .map(|(n, v)| Vertex::new(&v, &n))
                                    .collect()
                            } else {
                                return Err(format!("Model error in {}: mesh has no normals", path));
                            }
                        } else {
                            return Err(format!("Model error in {}: mesh has no vertices", path));
                        };
                        let indices: Vec<u16> = if let Some(indices_reader) = primitive_reader.read_indices() {
                            match indices_reader {
                                gltf::mesh::util::ReadIndices::U8(iter) => {
                                    iter.map(|v| v as u16).collect()
                                },
                                gltf::mesh::util::ReadIndices::U16(iter) => {
                                    iter.collect()
                                },
                                gltf::mesh::util::ReadIndices::U32(iter) => {
                                    iter.map(|v| v as u16).collect()
                                },
                            }
                        } else {
                            return Err(format!("Model error in {}: mesh has no indices", path));
                        };
                        return Ok(Model::new(vertices, indices));
                    }
                } else {
                    return Err(format!("Model error in {}: no 'root' node found", path));
                }
            } else {
                return Err(format!("Model error in {}: no default scene found", path));
            }
        }
        Err(format!("Failed to read GLTF file for model {}", path))
    }

    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Model {
        Model {
            vertices,
            indices,
        }
    }

    pub fn vertices_slice<'a>(&'a self) -> &'a[Vertex] {
        &self.vertices
    }

    pub fn indices_slice<'a>(&'a self) -> &'a[u16] {
        &self.indices
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