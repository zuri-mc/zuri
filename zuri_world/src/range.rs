use std::ops::Range;
use crate::pos::{BlockPos, ChunkPos};
use crate::subchunk::SUBCHUNKS_SIZE;

/// Represents the vertical range of a world.
#[derive(Copy, Clone)]
pub struct YRange {
    min: i16,
    max: i16,
}

impl YRange {
    pub fn new(min: i16, max: i16) -> YRange {
        if max <= min {
            panic!("range maximum has to be smaller than minimum");
        }
        if (max - min) % (SUBCHUNKS_SIZE as i16) != 0 {
            panic!("range height needs to be a multiple of {}", SUBCHUNKS_SIZE);
        }
        YRange { min, max }
    }

    pub fn min(&self) -> i16 {
        self.min
    }

    pub fn max(&self) -> i16 {
        self.max
    }

    pub fn height(&self) -> i16 {
        self.max - self.min
    }

    pub fn is_inside(&self, pos: impl Into<ChunkPos>) -> bool {
        let c = pos.into();
        self.min <= c.y() && self.max >= c.y()
    }
}

impl Into<Range<i16>> for YRange {
    fn into(self) -> Range<i16> {
        self.min..self.max
    }
}
