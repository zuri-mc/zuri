use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::DimensionDefinition;

/// A packet sent from the server to the client containing information about data-driven dimensions
/// that the server may have registered. This packet does not seem to be sent by default, rather
/// only being sent when any data-driven dimensions are registered.
#[derive(Debug, Clone)]
pub struct DimensionData {
    /// A list of data-driven dimension definitions registered on the server.
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
