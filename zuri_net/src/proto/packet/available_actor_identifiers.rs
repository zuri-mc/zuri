use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server at the start of the game to let the client know all entities that are
/// available on the server.
#[derive(Debug, Clone)]
pub struct AvailableActorIdentifiers {
    /// Network NBT serialised tag of all entity identifiers that are available in the server.
    pub serialised_entity_identifiers: Bytes,
}

impl PacketType for AvailableActorIdentifiers {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.serialised_entity_identifiers);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { serialised_entity_identifiers: reader.byte_slice() }
    }
}
