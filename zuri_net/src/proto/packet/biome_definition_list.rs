use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct BiomeDefinitionList {
    pub serialised_biome_definitions: Bytes,
}

impl PacketType for BiomeDefinitionList {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.serialised_biome_definitions);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            serialised_biome_definitions: reader.byte_slice(),
        }
    }
}
