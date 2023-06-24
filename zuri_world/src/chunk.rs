use glam::IVec2;
use std::iter;
use std::sync::Arc;

use crate::block;
use crate::block::{BlockBuilder, BlockMap, RuntimeId, ToRuntimeId};
use zuri_net::proto::io::Reader;

use crate::pos::ChunkIndex;
use crate::range::YRange;
use crate::sub_chunk::*;

/// A 16xYx16 column of blocks in a world.
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
pub struct Chunk {
    range: YRange,
    sub_chunks: Vec<SubChunk<8>>,

    block_map: Arc<BlockMap>,
}

impl Chunk {
    /// Creates a chunk filled with the provided runtime id. The height of the chunk can also be
    /// configured.
    #[must_use]
    pub fn empty(range: YRange, block_map: Arc<BlockMap>) -> Self {
        let air = BlockBuilder::new(block::AIR_ID).to_runtime_id(&block_map);

        Self {
            block_map,

            range,
            sub_chunks: iter::repeat(SubChunk::empty(air))
                .take((range.height() >> 4) as usize)
                .collect(),
        }
    }

    /// Returns the [BlockMap] that contains all registered blocks.
    pub fn block_map(&self) -> &Arc<BlockMap> {
        &self.block_map
    }

    /// The range of Y coordinates that this chunk contains. Some dimensions have taller or shorter
    /// chunks than other dimensions.
    pub fn range(&self) -> YRange {
        self.range
    }

    /// Returns the runtime id of the block located at the provided location in the chunk.
    #[must_use]
    pub fn at(&self, pos: ChunkIndex) -> RuntimeId {
        if !self.range.is_inside(pos) {
            panic!("chunk pos is outside of bounds"); // todo: maybe return an option
        }
        self.sub_chunks[self.subchunk_id(pos.y())].at(pos.into(), 0)
    }

    /// Sets the block at a position to a new block.
    pub fn set(&mut self, pos: ChunkIndex, val: impl ToRuntimeId) {
        if !self.range.is_inside(pos) {
            panic!("chunk pos is outside of bounds"); // todo: do we want to panic here
        }
        let id = self.subchunk_id(pos.y());
        self.sub_chunks[id].set(pos.into(), 0, val.to_runtime_id(&self.block_map));
    }

    /// Decodes a chunk from a [Reader].
    #[must_use = "Not using value of `Chunk::read` does nothing"]
    pub fn read(
        reader: &mut Reader,
        range: YRange,
        sub_chunk_count: u32,
        block_map: Arc<BlockMap>,
    ) -> Self {
        let air_rid = BlockBuilder::new(block::AIR_ID).to_runtime_id(&block_map);

        let mut sub_chunks = Vec::new();
        for _ in 0..(range.height() >> 4) {
            sub_chunks.push(SubChunk::empty(air_rid))
        }

        for mut sub_chunk_num in 0..sub_chunk_count {
            let sub_chunk = SubChunk::read(reader, &mut sub_chunk_num, &block_map);
            sub_chunks[sub_chunk_num as usize] = sub_chunk;
        }
        Self {
            range,
            sub_chunks,
            block_map,
        }
    }

    fn subchunk_id(&self, y: i16) -> usize {
        ((y - self.range.min()) >> 4) as usize
    }
}

/// A 2D vector referring to a chunk in the world. It is always a multiple of 16 of the position of
/// the first block in the chunk.
pub type ChunkPos = IVec2;
