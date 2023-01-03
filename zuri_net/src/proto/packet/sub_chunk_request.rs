use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::{Dimension, SubChunkOffset};

/// Requests specific sub-chunks from the server using a center point.
#[derive(Debug, Clone)]
pub struct SubChunkRequest {
    /// The dimension of the sub-chunks.
    pub dimension: Dimension,
    /// An absolute sub-chunk center point used as a base point for all sub-chunks requested. The X
    /// and Z coordinates represent the chunk coordinates, while the Y coordinate is the absolute
    /// sub-chunk index.
    pub position: IVec3,
    /// Requested offsets around the center point.
    pub offsets: Vec<SubChunkOffset>,
}

impl PacketType for SubChunkRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.block_pos(self.position);

        writer.u32(self.offsets.len() as u32);
        self.offsets.iter().for_each(|offset| offset.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            offsets: (0..reader.u32()).map(|_| SubChunkOffset::read(reader)).collect(),
        }
    }
}
