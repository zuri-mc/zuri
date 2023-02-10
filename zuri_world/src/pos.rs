use bevy::prelude::*;

/// A block position relative to the origin of a chunk. The x and z coordinates are always
/// guaranteed to be in the range `0..16`.
#[derive(Copy, Clone, Debug)]
pub struct ChunkIndex {
    x: u8,
    y: i16,
    z: u8,
}

impl ChunkIndex {
    pub fn new(x: u8, y: i16, z: u8) -> Self {
        if x >= 16 || z >= 16 {
            panic!("ChunkIndex out of bounds");
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

impl From<IVec3> for ChunkIndex {
    #[inline]
    fn from(value: IVec3) -> Self {
        Self {
            x: (value.x.rem_euclid(16)) as u8,
            y: value.y as i16,
            z: (value.z.rem_euclid(16)) as u8,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SubChunkIndex {
    x: u8,
    y: u8,
    z: u8,
}

impl SubChunkIndex {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        if x >= 16 || y >= 16 || z >= 16 {
            panic!("SubChunkIndex out of bounds");
        }
        Self { x, y, z }
    }

    #[inline]
    pub fn x(&self) -> u8 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> u8 {
        self.y
    }

    #[inline]
    pub fn z(&self) -> u8 {
        self.z
    }
}

impl From<IVec3> for SubChunkIndex {
    #[inline]
    fn from(value: IVec3) -> Self {
        Self {
            x: (value.x.rem_euclid(16)) as u8,
            y: (value.y.rem_euclid(16)) as u8,
            z: (value.z.rem_euclid(16)) as u8,
        }
    }
}

impl From<ChunkIndex> for SubChunkIndex {
    #[inline]
    fn from(value: ChunkIndex) -> Self {
        Self {
            x: value.x,
            y: (value.y.rem_euclid(16)) as u8,
            z: value.z,
        }
    }
}
