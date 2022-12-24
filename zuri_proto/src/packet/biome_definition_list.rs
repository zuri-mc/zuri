use bytes::Bytes;
use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
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
