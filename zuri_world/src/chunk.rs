use std::iter;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::pos::{BlockPos, ChunkPos};
use crate::range::YRange;
use crate::subchunk::*;

#[derive(Component)]
pub struct Chunk {
    range: YRange,
    sub_chunks: Vec<Option<SubChunk>>,
}

impl Chunk {
    pub fn empty(range: YRange) -> Self {
        Self {
            range,
            sub_chunks: iter::repeat(None).take((range.height() >> 4) as usize).collect(),
        }
    }

    pub fn at(&self, pos: ChunkPos) -> bool {
        if !self.range.is_inside(pos) {
            panic!("chunk pos is outside of bounds"); // todo: maybe return an option
        }
        if let Some(subchunk) = &self.sub_chunks[self.subchunk_id(pos.y())] {
            subchunk.at(pos.x(), ((pos.y() - self.range.min()) % 16) as u8, pos.z())
        } else {
            false
        }
    }

    pub fn set(&mut self, pos: ChunkPos, val: bool) {
        if !self.range.is_inside(pos) {
            panic!("chunk pos is outside of bounds"); // todo: do we want to panic here
        }
        let id = self.subchunk_id(pos.y());
        if let Some(subchunk) = &mut self.sub_chunks[id] {
            subchunk.set(pos.x(), ((pos.y() - self.range.min()) % 16) as u8, pos.z(), val)
        } else {
            let mut s = SubChunk::default();
            s.set(pos.x(), ((pos.y() - self.range.min()) % 16) as u8, pos.z(), val);
            self.sub_chunks[id] = Some(s);
        }
    }

    fn subchunk_id(&self, y: i16) -> usize {
        ((y - self.range.min()) >> 4) as usize
    }

    pub fn build_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut vertices = Vec::<[f32; 3]>::new();
        let mut triangles = Vec::<u32>::new();
        for x in 0..(SUBCHUNKS_SIZE as u8) {
            for y in self.range.min()..=self.range.max() {
                for z in 0..(SUBCHUNKS_SIZE as u8) {
                    if !self.at(ChunkPos::new(x, y, z)) {
                        continue;
                    }
                    let start_index = vertices.len() as u32;
                    if self.face_visible(x, y, z, 0, -1, 0) {
                        // Down 1
                        triangles.push(start_index);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 2);
                        // Down 2
                        triangles.push(start_index + 2);
                        triangles.push(start_index + 3);
                        triangles.push(start_index);
                    }
                    if self.face_visible(x, y, z, 0, 1, 0) {
                        // Up 1
                        triangles.push(start_index + 4 + 2);
                        triangles.push(start_index + 4 + 1);
                        triangles.push(start_index + 4);
                        // Up 2
                        triangles.push(start_index + 4);
                        triangles.push(start_index + 4 + 3);
                        triangles.push(start_index + 4 + 2);
                    }
                    if self.face_visible(x, y, z, 0, 0, -1) {
                        // North 1
                        triangles.push(start_index + 4 + 1);
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 0);
                        // North 2
                        triangles.push(start_index + 0);
                        triangles.push(start_index + 4 + 0);
                        triangles.push(start_index + 4 + 1);
                    }
                    if self.face_visible(x, y, z, 0, 0, 1) {
                        // South 1
                        triangles.push(start_index + 4 + 3);
                        triangles.push(start_index + 3);
                        triangles.push(start_index + 2);
                        // South 2
                        triangles.push(start_index + 2);
                        triangles.push(start_index + 4 + 2);
                        triangles.push(start_index + 4 + 3);
                    }
                    if self.face_visible(x, y, z, 1, 0, 0) {
                        // West 1
                        triangles.push(start_index + 4 + 2);
                        triangles.push(start_index + 2);
                        triangles.push(start_index + 1);
                        // West 2
                        triangles.push(start_index + 1);
                        triangles.push(start_index + 4 + 1);
                        triangles.push(start_index + 4 + 2);
                    }
                    if self.face_visible(x, y, z, -1, 0, 0) {
                        // East 1
                        triangles.push(start_index + 4 + 0);
                        triangles.push(start_index + 0);
                        triangles.push(start_index + 3);
                        // East 2
                        triangles.push(start_index + 3);
                        triangles.push(start_index + 4 + 3);
                        triangles.push(start_index + 4 + 0);
                    }

                    // Bottom half
                    vertices.push([x as f32, y as f32, z as f32]);
                    vertices.push([(x + 1) as f32, y as f32, z as f32]);
                    vertices.push([(x + 1) as f32, y as f32, (z + 1) as f32]);
                    vertices.push([x as f32, y as f32, (z + 1) as f32]);
                    // Top half
                    vertices.push([x as f32, (y + 1) as f32, z as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, z as f32]);
                    vertices.push([(x + 1) as f32, (y + 1) as f32, (z + 1) as f32]);
                    vertices.push([x as f32, (y + 1) as f32, (z + 1) as f32]);
                }
            }
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(Indices::U32(triangles)));

        mesh
    }

    fn face_visible(&self, x: u8, y: i16, z: u8, x_off: i8, y_off: i16, z_off: i8) -> bool {
        let max = SUBCHUNKS_SIZE as u8 - 1;
        if x_off < 0 && x == 0 || x_off > 0 && x == max || y_off < 0 && y == self.range.min() || y_off > 0 && y == self.range.max() || z_off < 0 && z == 0 || z_off > 0 && z == max {
            return true;
        }
        !self.at(ChunkPos::new((x as i8 + x_off) as u8, (y + y_off) as i16, (z as i8 + z_off) as u8))
    }
}
