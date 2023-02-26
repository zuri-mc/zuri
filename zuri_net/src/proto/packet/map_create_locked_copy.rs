use zuri_net_derive::proto;

use crate::proto::ints::VarI64;

/// Sent by the server to create a locked copy of one map into another map. In vanilla, it is used
/// in the cartography table to create a map that is locked and cannot be modified.
#[proto]
#[derive(Debug, Clone)]
pub struct MapCreateLockedCopy {
    /// ID of the map that is being copied. The locked copy will obtain all content that is visible
    /// on this map, except the content will not change.
    pub original_map_id: VarI64,
    /// ID of the map that holds the locked copy of the map that original_map_id points to. Its
    /// contents will be impossible to change.
    pub new_map_id: VarI64,
}
