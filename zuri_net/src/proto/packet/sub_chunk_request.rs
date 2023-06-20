use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

use crate::proto::io::BlockPos;
use crate::proto::types::world::{Dimension, SubChunkOffset};

/// Requests specific sub-chunks from the server using a center point.
#[proto]
#[derive(Debug, Clone)]
pub struct SubChunkRequest {
    /// The dimension of the sub-chunks.
    #[enum_header(VarI32)]
    pub dimension: Dimension,
    /// An absolute sub-chunk center point used as a base point for all sub-chunks requested. The X
    /// and Z coordinates represent the chunk coordinates, while the Y coordinate is the absolute
    /// sub-chunk index.
    pub position: BlockPos,
    /// Requested offsets around the center point.
    #[len_type(u32)]
    pub offsets: Vec<SubChunkOffset>,
}
