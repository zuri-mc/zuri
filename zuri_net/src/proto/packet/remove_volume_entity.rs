use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

use crate::proto::types::world::Dimension;

/// Sent to the client to indicate that a volume entity has been removed.
#[proto]
#[derive(Debug, Clone)]
pub struct RemoveVolumeEntity {
    /// The entity runtime ID of the volume entity that was removed.
    pub entity_runtime_id: u64,
    /// The dimension that the volume entity was in.
    #[enum_header(VarI32)]
    pub dimension: Dimension,
}
