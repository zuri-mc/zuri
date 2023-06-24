use std::ops::{Range, RangeInclusive};

use crate::pos::ChunkIndex;
use crate::sub_chunk::SUBCHUNK_SIZE;

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
        if (max - min + 1) % (SUBCHUNK_SIZE as i16) != 0 {
            panic!("range height needs to be a multiple of {}", SUBCHUNK_SIZE);
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

    pub fn iter(&self) -> RangeInclusive<i16> {
        (self.min..=self.max).into_iter()
    }
}

impl IntoIterator for YRange {
    type Item = i16;
    type IntoIter = RangeInclusive<i16>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<YRange> for Range<i16> {
    fn from(value: YRange) -> Self {
        value.min()..value.max()
    }
}
