use crate::proto::ints::{VarI32, VarI64, VarU64};
use glam::Vec3;
use zuri_net_derive::proto;

/// Sent by the server to the client to make a painting entity show up. It is one of the few
/// entities that cannot be sent using the AddActor packet.
#[proto]
#[derive(Debug, Clone)]
pub struct AddPainting {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// entity out for this field.
    pub entity_unique_id: VarI64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot
    /// see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The facing direction of the painting.
    pub direction: VarI32,
    /// The title of the painting. It specifies the motive of the painting. The title of the
    /// painting must be valid.
    pub title: String,
}
