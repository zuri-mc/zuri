use crate::world::component::Geometry;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use zuri_world::block::component::ComponentStorage;
use zuri_world::chunk::Chunk;
use zuri_world::pos::ChunkIndex;

pub fn build_mesh(chunk: &Chunk) -> Mesh {
    let geometries = chunk.block_map().components::<Geometry>();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut uv = Vec::<[f32; 2]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();
    let mut triangles = Vec::<u32>::new();
    for x in 0..(16 as u8) {
        for y in chunk.range() {
            for z in 0..(16 as u8) {
                let mut start_index = vertices.len() as u32;
                if let Some(geo) = geometries.get(chunk.at(ChunkIndex::new(x, y, z))) {
                    // The block has a custom geometry (it is not a cube) for now, this means
                    // it is treated as visible.
                    let geo_vertices = geo.mesh.attribute(Mesh::ATTRIBUTE_POSITION);
                    if geo_vertices.is_some() {
                        if let VertexAttributeValues::Float32x3(positions) = geo_vertices.unwrap() {
                            for vertex in positions {
                                vertices.push([
                                    vertex[0] + x as f32,
                                    vertex[1] + y as f32,
                                    vertex[2] + z as f32,
                                ]);
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    let geo_normals = geo.mesh.attribute(Mesh::ATTRIBUTE_NORMAL);
                    if geo_normals.is_some() {
                        if let VertexAttributeValues::Float32x3(x) = geo_normals.unwrap() {
                            for normal in x {
                                normals.push(*normal);
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    let geo_uvs = geo.mesh.attribute(Mesh::ATTRIBUTE_UV_0);
                    if geo_uvs.is_some() {
                        if let VertexAttributeValues::Float32x2(x) = geo_uvs.unwrap() {
                            for val in x {
                                uv.push(*val);
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    let trias_opt = geo.mesh.indices();
                    if let Some(trias) = trias_opt {
                        for tria in trias.iter() {
                            triangles.push(tria as u32 + start_index);
                        }
                    }
                    continue;
                }
                if face_visible(chunk, geometries, x, y, z, 0, -1, 0) {
                    vertices.push([x as f32, y as f32, z as f32]);
                    vertices.push([(x + 1) as f32, y as f32, z as f32]);
                    vertices.push([x as f32, y as f32, (z + 1) as f32]);
                    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);

                    uv.push([0., 0.]);
                    uv.push([1., 0.]);
                    uv.push([0., 1.]);
                    uv.push([1., 1.]);

                    normals.push([0., -1., 0.]);
                    normals.push([0., -1., 0.]);
                    normals.push([0., -1., 0.]);
                    normals.push([0., -1., 0.]);

                    // Down 1
                    triangles.push(start_index + 0);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);
                    // Down 2
                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 3);

                    start_index += 4;
                }
                if face_visible(chunk, geometries, x, y, z, 0, 1, 0) {
                    vertices.push([x as f32, (y + 1) as f32, z as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);
                    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);

                    uv.push([0., 0.]);
                    uv.push([1., 0.]);
                    uv.push([0., 1.]);
                    uv.push([1., 1.]);

                    normals.push([0., 1.0, 0.]);
                    normals.push([0., 1.0, 0.]);
                    normals.push([0., 1.0, 0.]);
                    normals.push([0., 1.0, 0.]);

                    // Up 1
                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 0);
                    // Up 2
                    triangles.push(start_index + 3);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);

                    start_index += 4;
                }
                if face_visible(chunk, geometries, x, y, z, 0, 0, -1) {
                    vertices.push([x as f32, y as f32, z as f32]);
                    vertices.push([(x + 1) as f32, y as f32, z as f32]);
                    vertices.push([x as f32, (y + 1) as f32, z as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);

                    uv.push([0., 0.]);
                    uv.push([1., 0.]);
                    uv.push([0., 1.]);
                    uv.push([1., 1.]);

                    normals.push([0., 0., -1.0]);
                    normals.push([0., 0., -1.0]);
                    normals.push([0., 0., -1.0]);
                    normals.push([0., 0., -1.0]);

                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 0);

                    triangles.push(start_index + 3);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);

                    start_index += 4;
                }
                if face_visible(chunk, geometries, x, y, z, 0, 0, 1) {
                    vertices.push([x as f32, y as f32, (z + 1) as f32]);
                    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);
                    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);

                    uv.push([0., 0.]);
                    uv.push([1., 0.]);
                    uv.push([0., 1.]);
                    uv.push([1., 1.]);

                    normals.push([0., 0., 1.0]);
                    normals.push([0., 0., 1.0]);
                    normals.push([0., 0., 1.0]);
                    normals.push([0., 0., 1.0]);
                    triangles.push(start_index + 0);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);

                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 3);

                    start_index += 4;
                }
                if face_visible(chunk, geometries, x, y, z, 1, 0, 0) {
                    vertices.push([(x + 1) as f32, y as f32, z as f32]);
                    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);

                    uv.push([0., 0.]);
                    uv.push([1., 0.]);
                    uv.push([0., 1.]);
                    uv.push([1., 1.]);

                    normals.push([1., 0., 0.]);
                    normals.push([1., 0., 0.]);
                    normals.push([1., 0., 0.]);
                    normals.push([1., 0., 0.]);

                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 0);

                    triangles.push(start_index + 3);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);

                    start_index += 4;
                }
                if face_visible(chunk, geometries, x, y, z, -1, 0, 0) {
                    vertices.push([x as f32, y as f32, z as f32]);
                    vertices.push([x as f32, y as f32, (z + 1) as f32]);
                    vertices.push([x as f32, (y + 1) as f32, z as f32]);
                    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);

                    uv.push([1., 1.]);
                    uv.push([1., 0.]);
                    uv.push([0., 1.]);
                    uv.push([0., 0.]);

                    normals.push([-1., 0., 0.]);
                    normals.push([-1., 0., 0.]);
                    normals.push([-1., 0., 0.]);
                    normals.push([-1., 0., 0.]);

                    triangles.push(start_index + 0);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);

                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 3);
                }
            }
        }
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(triangles)));

    mesh
}

fn face_visible(
    chunk: &Chunk,
    geometries: &ComponentStorage<Geometry>,
    x: u8,
    y: i16,
    z: u8,
    x_off: i8,
    y_off: i16,
    z_off: i8,
) -> bool {
    let max = 16 as u8 - 1;
    if x_off < 0 && x == 0
        || x_off > 0 && x == max
        || y_off < 0 && y == chunk.range().min()
        || y_off > 0 && y == chunk.range().max()
        || z_off < 0 && z == 0
        || z_off > 0 && z == max
    {
        return true;
    }
    let neighbour_geo = geometries.get(chunk.at(ChunkIndex::new(
        (x as i8 + x_off) as u8,
        (y + y_off) as i16,
        (z as i8 + z_off) as u8,
    )));
    neighbour_geo.is_some() // todo: have a smarter system for this
}
