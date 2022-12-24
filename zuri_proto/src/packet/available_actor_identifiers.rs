use bytes::Bytes;
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct AvailableActorIdentifiers {
    pub serialised_entity_identifiers: Bytes,
}

impl Packet for AvailableActorIdentifiers {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.serialised_entity_identifiers);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            serialised_entity_identifiers: reader.byte_slice(),
        }
    }
}
