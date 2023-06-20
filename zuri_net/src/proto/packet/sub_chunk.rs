use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::{Dimension, SubChunkEntry};

/// Sends data about multiple sub-chunks around a center point.
#[derive(Debug, Clone)]
pub struct SubChunk {
    /// Whether client chunk caching is enabled or not.
    pub cache_enabled: bool,
    /// The dimension the sub-chunks are in.
    pub dimension: Dimension,
    /// An absolute sub-chunk center point that every SubChunkRequest uses as a reference.
    pub position: IVec3,
    /// Sub-chunk entries relative to the center point.
    pub sub_chunk_entries: Vec<SubChunkEntry>,
}

impl PacketType for SubChunk {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.cache_enabled);
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.block_pos(self.position);
        writer.u32(self.sub_chunk_entries.len() as u32);
        self.sub_chunk_entries
            .iter()
            .for_each(|entry| entry.write(writer, self.cache_enabled));
    }

    fn read(reader: &mut Reader) -> Self {
        let cache_enabled = reader.bool();
        Self {
            cache_enabled,
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            sub_chunk_entries: (0..reader.u32())
                .map(|_| SubChunkEntry::read(reader, cache_enabled))
                .collect(),
        }
    }
}
