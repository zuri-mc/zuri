use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the server to let the client know what entity type it was last hurt by. At this moment,
/// the packet is useless and should not be used. There is no behaviour that depends on if this
/// packet is sent or not.
#[proto]
#[derive(Debug, Clone)]
pub struct SetLastHurtBy {
    /// The numerical type of the entity that the player was last hurt by.
    pub entity_type: VarI32,
}
