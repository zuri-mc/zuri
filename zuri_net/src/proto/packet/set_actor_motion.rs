use glam::Vec3;

use zuri_net_derive::proto;

use crate::proto::ints::VarU64;

/// Sent by the server to change the client-side velocity of an entity. It is usually used in
/// combination with server-side movement calculation.
#[proto]
#[derive(Debug, Clone)]
pub struct SetActorMotion {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The new velocity the entity gets. This velocity will initiate the client-side movement of
    /// the entity.
    pub velocity: Vec3,
}
