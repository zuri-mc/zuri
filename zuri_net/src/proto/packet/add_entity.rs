use zuri_net_derive::proto;

use crate::proto::ints::VarU64;

/// Sent by the server to the client. Its function is not entirely clear: It does not add an entity
/// in the sense of an in-game entity, but has to do with the ECS that Minecraft uses.
#[proto]
#[derive(Debug, Clone)]
pub struct AddEntity {
    pub entity_network_id: VarU64,
}
