use bytes::Bytes;

use zuri_net_derive::proto;

/// Sent by the server at the start of the game to let the client know all entities that are
/// available on the server.
#[proto]
#[derive(Debug, Clone)]
pub struct AvailableActorIdentifiers {
    /// Network NBT serialised tag of all entity identifiers that are available in the server.
    pub serialised_entity_identifiers: Bytes,
}
