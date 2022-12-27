use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct AvailableActorIdentifiers {
    pub serialised_entity_identifiers: Bytes,
}

impl PacketType for AvailableActorIdentifiers {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.serialised_entity_identifiers);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            serialised_entity_identifiers: reader.byte_slice(),
        }
    }
}
