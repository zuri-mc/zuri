use std::collections::HashMap;
use std::iter;

use crate::block::component::geometry::Geometry;
use crate::block::component::ComponentStorage;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_render::mesh::VertexAttributeValues;
use zuri_net::proto::io::Reader;

use crate::pos::ChunkIndex;
use crate::range::YRange;
use crate::sub_chunk::*;

/// A 16xYx16 column of blocks in a world.
#[derive(Component)]
pub struct Chunk {
    _air_rid: u32,

    range: YRange,
    sub_chunks: Vec<Option<Box<SubChunk<8>>>>,
}

impl Chunk {
    /// Creates a chunk filled with the provided runtime id. The height of the chunk can also be
    /// configured.
    pub fn empty(range: YRange, _air_rid: u32) -> Self {
        Self {
            _air_rid,
            range,
            sub_chunks: iter::repeat(None)
                .take((range.height() >> 4) as usize)
                .collect(),
        }
    }

    /// Returns the runtime id of the block located at the provided location in the chunk.
    pub fn at(&self, pos: ChunkIndex) -> u32 {
        if !self.range.is_inside(pos) {
            panic!("chunk pos is outside of bounds"); // todo: maybe return an option
        }
        if let Some(subchunk) = &self.sub_chunks[self.subchunk_id(pos.y())] {
            subchunk.at(pos.into(), 0)
        } else {
            self._air_rid
        }
    }

    pub fn set(&mut self, pos: ChunkIndex, val: u32) {
        if !self.range.is_inside(pos) {
            panic!("chunk pos is outside of bounds"); // todo: do we want to panic here
        }
        let id = self.subchunk_id(pos.y());
        if let Some(subchunk) = &mut self.sub_chunks[id] {
            subchunk.set(pos.into(), 0, val)
        } else {
            let mut s = Box::new(SubChunk::empty(self._air_rid));
            s.set(pos.into(), 0, val);
            self.sub_chunks[id] = Some(s);
        }
    }

    pub fn read(reader: &mut Reader, range: YRange, sub_chunk_count: u32, air_rid: u32) -> Self {
        let mut chunk = Self::empty(range, air_rid);

        for mut sub_chunk_num in 0..sub_chunk_count {
            let sub_chunk = SubChunk::read(reader, &mut sub_chunk_num, air_rid);
            chunk.sub_chunks[sub_chunk_num as usize] = Some(Box::new(sub_chunk));
        }
        chunk
    }

    fn subchunk_id(&self, y: i16) -> usize {
        ((y - self.range.min()) >> 4) as usize
    }

    pub fn build_mesh(&self, geometries: &ComponentStorage<Geometry>) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut uv = Vec::<[f32; 2]>::new();
        let mut normals = Vec::<[f32; 3]>::new();
        let mut vertices = Vec::<[f32; 3]>::new();
        let mut triangles = Vec::<u32>::new();
        for x in 0..(16 as u8) {
            for y in self.range.min()..=self.range.max() {
                for z in 0..(16 as u8) {
                    let mut start_index = vertices.len() as u32;
                    if let Some(geo) = geometries.get(self.at(ChunkIndex::new(x, y, z))) {
                        // The block has a custom geometry (it is not a cube) for now, this means
                        // it is treated as visible.
                        let geo_vertices = geo.mesh.attribute(Mesh::ATTRIBUTE_POSITION);
                        if geo_vertices.is_some() {
                            if let VertexAttributeValues::Float32x3(positions) =
                                geo_vertices.unwrap()
                            {
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
                    if self.face_visible(geometries, x, y, z, 0, -1, 0) {
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
                    if self.face_visible(geometries, x, y, z, 0, 1, 0) {
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
                    if self.face_visible(geometries, x, y, z, 0, 0, -1) {
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
                    if self.face_visible(geometries, x, y, z, 0, 0, 1) {
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
                    if self.face_visible(geometries, x, y, z, 1, 0, 0) {
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
                    if self.face_visible(geometries, x, y, z, -1, 0, 0) {
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
        &self,
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
            || y_off < 0 && y == self.range.min()
            || y_off > 0 && y == self.range.max()
            || z_off < 0 && z == 0
            || z_off > 0 && z == max
        {
            return true;
        }
        let neighbour_geo = geometries.get(self.at(ChunkIndex::new(
            (x as i8 + x_off) as u8,
            (y + y_off) as i16,
            (z as i8 + z_off) as u8,
        )));
        neighbour_geo.is_some() // todo: have a smarter system for this
    }
}

/// A 2D vector referring to a chunk in the world. It is always a multiple of 16 of the position of
/// the first block in the chunk.
pub type ChunkPos = IVec2;

/// Keeps track of all chunks present in the world.
/// Contains a reference to the entity that contains a chunk's data given a certain chunk position.
#[derive(Resource, Default, Debug)]
pub struct ChunkManager {
    chunks: HashMap<ChunkPos, Entity>,
}

impl ChunkManager {
    /// Returns the chunk entity associated to the chunk position passed as an argument.
    /// If there is no loaded chunk at the position, None is returned.
    pub fn get(&self, chunk_pos: ChunkPos) -> Option<Entity> {
        self.chunks.get(&chunk_pos).copied()
    }

    /// Changes what chunk entity a chunk position refers to.
    /// If None is passed, the chunk is removed from the chunk manager (but not from the world).
    pub fn set(&mut self, chunk_pos: ChunkPos, val: Option<Entity>) -> Option<Entity> {
        match val {
            Some(inner) => self.chunks.insert(chunk_pos, inner),
            None => self.chunks.remove(&chunk_pos),
        }
    }

    /// Returns the chunk in which the provided world position is in (if it is loaded).
    /// The provided Y-value does not have an effect on the result.
    pub fn at_pos(&self, world_pos: Vec3) -> Option<Entity> {
        self.get(IVec2::new(
            world_pos.x.floor() as i32 >> 4,
            world_pos.z.floor() as i32 >> 4,
        ))
    }

    /// Returns the chunk in which the provided block position is in (if it is loaded).
    /// The provided Y-value does not have an effect on the result.
    pub fn at_block_pos(&self, world_pos: IVec3) -> Option<Entity> {
        self.get(IVec2::new(world_pos.x >> 4, world_pos.z >> 4))
    }

    /// Removes all known chunks from the chunk manager. Does NOT remove these entities from the
    /// world.
    pub fn clear(&mut self) {
        self.chunks.clear();
    }
}
