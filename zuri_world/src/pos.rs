use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct BlockPos {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl From<Vec3> for BlockPos {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x.floor() as i32,
            y: value.y.floor() as i16,
            z: value.z.floor() as i32,
        }
    }
}

impl Into<Vec3> for BlockPos {
    fn into(self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// A block position relative to the origin of a chunk.
#[derive(Copy, Clone)]
pub struct ChunkPos {
    x: u8,
    y: i16,
    z: u8,
}

impl ChunkPos {
    pub fn new(x: u8, y: i16, z: u8) -> Self {
        if x >= 16 || z >= 16 {
            panic!("chunk position out of bounds");
        }
        Self { x, y, z }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> i16 {
        self.y
    }

    pub fn z(&self) -> u8 {
        self.z
    }
}

impl From<BlockPos> for ChunkPos {
    fn from(value: BlockPos) -> Self {
        Self {
            x: (value.x % 16) as u8,
            y: value.y,
            z: (value.z % 16) as u8,
        }
    }
}
