use zuri_net_derive::proto;

use crate::proto::ints::VarI64;

/// Sent by the server to remove an entity that currently exists in the world from the client-side.
/// Sending this packet if the client cannot already see this entity will have no effect.
#[proto]
#[derive(Debug, Clone)]
pub struct RemoveActor {
    /// The unique ID of the entity to be removed. The unique ID is a value that remains consistent
    /// across different sessions of the same world, but most servers simply fill the runtime ID of
    /// the entity out for this field.
    pub entity_unique_id: VarI64,
}
