pub const SUBCHUNKS_SIZE: usize = 16;
pub const SUBCHUNKS_BLOCK_COUNT: usize = SUBCHUNKS_SIZE * SUBCHUNKS_SIZE * SUBCHUNKS_SIZE;

#[derive(Clone)]
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
}
