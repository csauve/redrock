use cgmath::{Matrix4, Matrix3, Vector3, Vector2, prelude::*};
use std::vec::Vec;
use gltf;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    position: Vector3<f32>,
    normal: Vector3<f32>,
    tangent: Vector3<f32>,
    bitangent: Vector3<f32>,
    uv: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, tangent: Vector3<f32>, bitangent: Vector3<f32>, uv: Vector2<f32>) -> Vertex {
        Vertex {
            position,
            normal,
            tangent,
            bitangent,
            uv,
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

                        let vertices: Vec<Vertex> = match (
                            primitive_reader.read_positions(),
                            primitive_reader.read_normals(),
                            primitive_reader.read_tangents(),
                            primitive_reader.read_tex_coords(0),
                        ) {
                            (
                                Some(positions),
                                Some(normals),
                                Some(tangents),
                                Some(uvs),
                            ) => {
                                let normals: Vec<[f32; 3]> = normals.collect();
                                let tangents: Vec<[f32; 4]> = tangents.collect();
                                let uvs: Vec<[f32; 2]> = match uvs {
                                    gltf::mesh::util::ReadTexCoords::U8(uvs) => {
                                        uvs.map(|uv| {
                                            [uv[0] as f32, uv[1] as f32]
                                        }).collect()
                                    },
                                    gltf::mesh::util::ReadTexCoords::U16(uvs) => {
                                        uvs.map(|uv| {
                                            [uv[0] as f32, uv[1] as f32]
                                        }).collect()
                                    },
                                    gltf::mesh::util::ReadTexCoords::F32(uvs) => {
                                        uvs.collect()
                                    },
                                };
                                positions.enumerate().map(|(i, pos)| {
                                    let position = Vector3::new(pos[0], pos[1], pos[2]);
                                    let normal = Vector3::new(normals[i][0], normals[i][1], normals[i][2]);
                                    let tangent = Vector3::new(tangents[i][0], tangents[i][1], tangents[i][2]);
                                    let bitangent: Vector3<f32> = normal.cross(tangent) * tangents[i][3];
                                    let uv = Vector2::new(uvs[i][0], uvs[i][1]);
                                    Vertex::new(
                                        position,
                                        normal,
                                        tangent,
                                        bitangent,
                                        uv,
                                    )
                                }).collect()
                            },
                            (p, n, t, uv) => {
                                return Err(format!(
                                    "Model error in {}: mesh has incomplete vertex data; positions={}, normals={}, tangents={}, uvs={}",
                                    path,
                                    p.is_some(),
                                    n.is_some(),
                                    t.is_some(),
                                    uv.is_some(),
                                ));
                            }
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
    }
}