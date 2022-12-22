use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

const SUBCHUNKS_SIZE: usize = 16;
const SUBCHUNKS_BLOCK_COUNT: usize = SUBCHUNKS_SIZE * SUBCHUNKS_SIZE * SUBCHUNKS_SIZE;

pub struct SubChunk {
    // todo: store actual blocks instead of booleans
    blocks: [bool; SUBCHUNKS_BLOCK_COUNT],
}

impl Default for SubChunk {
    fn default() -> Self {
        Self {
            blocks: [false; SUBCHUNKS_BLOCK_COUNT],
        }
    }
}

impl SubChunk {
    #[inline]
    fn require_inside(x: u8, y: u8, z: u8) {
        if x >= SUBCHUNKS_SIZE as u8 || y >= SUBCHUNKS_SIZE as u8 || z >= SUBCHUNKS_SIZE as u8 {
            panic!("subchunk position out of bounds");
        }
    }

    #[inline]
    fn index(x: u8, y: u8, z: u8) -> usize {
        (x as usize) + ((z as usize) << 4) + ((y as usize) << 8)
    }

    pub fn at(&self, x: u8, y: u8, z: u8) -> bool {
        Self::require_inside(x, y, z);
        self.blocks[Self::index(x, y, z)]
    }

    pub fn set(&mut self, x: u8, y: u8, z: u8, val: bool) {
        Self::require_inside(x, y, z);
        self.blocks[Self::index(x, y, z)] = val;
    }

    pub fn build_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut vertices = Vec::<[f32; 3]>::new();
        let mut triangles = Vec::<u32>::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if !self.at(x, y, z) {
                        continue;
                    }
                    let start_index = vertices.len() as u32;
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

                    // Bottom 1
                    triangles.push(start_index);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 2);
                    // Bottom 2
                    triangles.push(start_index + 2);
                    triangles.push(start_index + 3);
                    triangles.push(start_index);

                    // Top 1
                    triangles.push(start_index + 4 + 2);
                    triangles.push(start_index + 4 + 1);
                    triangles.push(start_index + 4);
                    // Top 2
                    triangles.push(start_index + 4 );
                    triangles.push(start_index + 4 + 3);
                    triangles.push(start_index + 4 + 2);

                    // Side1 1
                    triangles.push(start_index + 4 + 1);
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 0);
                    // Side1 2
                    triangles.push(start_index + 0);
                    triangles.push(start_index + 4 + 0);
                    triangles.push(start_index + 4 + 1);

                    // Side2 1
                    triangles.push(start_index + 4 + 2);
                    triangles.push(start_index + 2);
                    triangles.push(start_index + 1);
                    // Side2 2
                    triangles.push(start_index + 1);
                    triangles.push(start_index + 4 + 1);
                    triangles.push(start_index + 4 + 2);

                    // Side3 1
                    triangles.push(start_index + 4 + 3);
                    triangles.push(start_index + 3);
                    triangles.push(start_index + 2);
                    // Side3 2
                    triangles.push(start_index + 2);
                    triangles.push(start_index + 4 + 2);
                    triangles.push(start_index + 4 + 3);

                    // Side4 1
                    triangles.push(start_index + 4 + 0);
                    triangles.push(start_index + 0);
                    triangles.push(start_index + 3);
                    // Side4 2
                    triangles.push(start_index + 3);
                    triangles.push(start_index + 4 + 3);
                    triangles.push(start_index + 4 + 0);
                }
            }
        }
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.set_indices(Some(Indices::U32(triangles)));

        mesh
    }
}
