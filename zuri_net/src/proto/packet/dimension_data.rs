use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::DimensionDefinition;

#[derive(Debug)]
pub struct DimensionData {
    pub definitions: Vec<DimensionDefinition>,
}

impl PacketType for DimensionData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.definitions.len() as u32);
        self.definitions.iter().for_each(|d| d.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { definitions: (0..reader.var_u32()).map(|_| DimensionDefinition::read(reader)).collect() }
    }
}
