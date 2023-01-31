use zuri_net_derive::packet;

use crate::proto::ints::VarU64;

/// Sent by the server to the client. Its function is not entirely clear: It does not remove an
/// entity in the sense of an in-game entity, but has to do with the ECS that Minecraft uses.
#[packet]
#[derive(Debug, Clone)]
pub struct RemoveEntity {
    /// The network ID of the entity that should be removed.
    pub entity_network_id: VarU64,
}
