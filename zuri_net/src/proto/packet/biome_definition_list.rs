use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to let the client know all biomes that are available and implemented on the
/// server side. It is much like the AvailableActorIdentifiers packet, but instead for biomes.
#[derive(Debug, Clone)]
pub struct BiomeDefinitionList {
    /// Network NBT serialised tag of all definitions of biomes that are available on the server.
    pub serialised_biome_definitions: Bytes,
}

impl PacketType for BiomeDefinitionList {
    fn write(&self, writer: &mut Writer) {
        writer.bytes(&self.serialised_biome_definitions);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { serialised_biome_definitions: reader.bytes() }
    }
}
