use crate::world::component::Geometry;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use zuri_world::block::BlockMap;
use zuri_world::chunk::Chunk;
use zuri_world::pos::ChunkIndex;

// todo: algorithm is probably quite slow, I came up with this on the spot.
/// Constructs an optimised mesh for a chunk.
pub fn build_mesh(block_map: &BlockMap, chunk: &Chunk) -> Mesh {
    let geometries = block_map.components::<Geometry>();

    let mut chunk_uv = Vec::<[f32; 2]>::new();
    let mut chunk_normals = Vec::<[f32; 3]>::new();
    let mut chunk_vertices = Vec::<[f32; 3]>::new();
    let mut chunk_triangles = Vec::<u32>::new();

    // The index of the first vertex of the current mesh in the loop that follows.
    let mut base_index = 0;
    // Contains a mask for each vertex that holds info on whether it may be removed. It is
    // determined by looking at whether it is possibly part of a face that is not visible.
    let mut vertex_vis_mask = Vec::new();

    // todo: cache locality
    for x in 0..16 {
        for z in 0..16 {
            for y in chunk.range() {
                let y = y as i32;
                let runtime_id = chunk.at(ChunkIndex::new(x as u8, y as i16, z as u8));
                // Blocks without a geometry can be ignored.
                let Some(geometry) = geometries.get(runtime_id) else { continue };

                if geometry.mesh.primitive_topology() != PrimitiveTopology::TriangleList {
                    panic!(
                        "Block geometry has unsupported PrimitiveTopology: {:?}",
                        geometry.mesh.primitive_topology()
                    );
                }

                macro_rules! is_visible {
                    (($x:expr, $y:expr, $z:expr), $field:ident) => {
                        'expanded: {
                            let nx = x + ($x);
                            let ny = y + ($y);
                            let nz = z + ($z);
                            if nx < 0 || nx >= 16 {
                                break 'expanded true;
                            }
                            if ny < 0 || ny >= chunk.range().max().into() {
                                break 'expanded true;
                            }
                            if nz < 0 || nz >= 16 {
                                break 'expanded true;
                            }
                            geometries
                                .get(chunk.at(ChunkIndex::new(nx as u8, ny as i16, nz as u8)))
                                .map(|v| !v.$field)
                                .unwrap_or(true)
                        }
                    };
                }
                let x_pos_visible = is_visible!((1, 0, 0), x_neg_solid);
                let x_neg_visible = is_visible!((-1, 0, 0), x_pos_solid);
                let y_pos_visible = is_visible!((0, 1, 0), y_neg_solid);
                let y_neg_visible = is_visible!((0, -1, 0), y_pos_solid);
                let z_pos_visible = is_visible!((0, 0, 1), z_neg_solid);
                let z_neg_visible = is_visible!((0, 0, -1), z_pos_solid);

                let positions = geometry.mesh.attribute(Mesh::ATTRIBUTE_POSITION);
                let Some(VertexAttributeValues::Float32x3(positions)) = positions else {
                    // todo: check if this is ever reasonably possible.
                    panic!("Unexpected value for ATTRIBUTE_POSITION: {positions:?}");
                };

                let uvs = geometry.mesh.attribute(Mesh::ATTRIBUTE_UV_0);
                let Some(VertexAttributeValues::Float32x2(uvs)) = uvs else {
                    // todo: check if this is ever reasonably possible.
                    panic!("Unexpected value for ATTRIBUTE_UV_0: {uvs:?}");
                };

                let normals = geometry.mesh.attribute(Mesh::ATTRIBUTE_NORMAL);
                let Some(VertexAttributeValues::Float32x3(normals)) = normals else {
                    // todo: check if this is ever reasonably possible.
                    panic!("Unexpected value for ATTRIBUTE_NORMAL: {normals:?}");
                };

                vertex_vis_mask.clear();

                for vertex in positions {
                    let vx = vertex[0];
                    let vy = vertex[1];
                    let vz = vertex[2];

                    let mut mask = 0u8;
                    if vy >= -f32::EPSILON
                        && vy <= 1. + f32::EPSILON
                        && vz >= -f32::EPSILON
                        && vz <= 1. + f32::EPSILON
                    {
                        if (vx - 1.).abs() <= f32::EPSILON && !x_pos_visible {
                            mask |= 0b1;
                        } else if vx.abs() <= f32::EPSILON && !x_neg_visible {
                            mask |= 0b10;
                        }
                    }
                    if vx >= -f32::EPSILON
                        && vx <= 1. + f32::EPSILON
                        && vz >= -f32::EPSILON
                        && vz <= 1. + f32::EPSILON
                    {
                        if (vy - 1.).abs() <= f32::EPSILON && !y_pos_visible {
                            mask |= 0b100;
                        } else if vy.abs() <= f32::EPSILON && !y_neg_visible {
                            mask |= 0b1000;
                        }
                    }
                    if vx >= -f32::EPSILON
                        && vx <= 1. + f32::EPSILON
                        && vy >= -f32::EPSILON
                        && vy <= 1. + f32::EPSILON
                    {
                        if (vz - 1.).abs() <= f32::EPSILON && !z_pos_visible {
                            mask |= 0b10000;
                        } else if vz.abs() <= f32::EPSILON && !z_neg_visible {
                            mask |= 0b100000;
                        }
                    }
                    vertex_vis_mask.push(mask);
                }

                let faces = FaceIterator {
                    inner: geometry.mesh.indices().unwrap().iter(),
                };

                for face in faces {
                    let mut mask_sum = !0u8;
                    for vertex in face {
                        mask_sum &= vertex_vis_mask[vertex];
                    }

                    if mask_sum != 0 {
                        continue;
                    }

                    for old_index in face {
                        let new_index = base_index;
                        base_index += 1;

                        // Also add properties.
                        let pos = positions[old_index];
                        chunk_vertices.push([
                            pos[0] + x as f32,
                            pos[1] + y as f32,
                            pos[2] + z as f32,
                        ]);
                        chunk_uv.push(uvs[old_index]);
                        chunk_normals.push(normals[old_index]);

                        chunk_triangles.push(new_index);
                    }
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, chunk_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, chunk_uv);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, chunk_normals);
    mesh.set_indices(Some(Indices::U32(chunk_triangles)));
    mesh
}

/// Helper struct to iterate over all triangles off a mesh.
struct FaceIterator<I: Iterator<Item = usize>> {
    inner: I,
}

impl<I: Iterator<Item = usize>> Iterator for FaceIterator<I> {
    type Item = [usize; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let mut array = [0, 0, 0];
        for i in 0..3 {
            let next = self.inner.next()?;
            array[i] = next;
        }
        Some(array)
    }
}
