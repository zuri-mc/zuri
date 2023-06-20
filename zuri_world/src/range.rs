use std::ops::Range;

use crate::pos::ChunkIndex;

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
        if (max - min + 1) % (16 as i16) != 0 {
            panic!("range height needs to be a multiple of {}", 16);
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
        self.max - self.min + 1
    }

    pub fn is_inside(&self, pos: impl Into<ChunkIndex>) -> bool {
        let c = pos.into();
        self.min <= c.y() && self.max >= c.y()
    }
}

impl Into<Range<i16>> for YRange {
    fn into(self) -> Range<i16> {
        self.min..self.max
    }
}
