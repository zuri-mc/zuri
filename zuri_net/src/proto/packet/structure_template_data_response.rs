use num_traits::{FromPrimitive, ToPrimitive};
use zuri_nbt::{Value, encoding::NetworkLittleEndian};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::structure::StructureTemplateDataRequestType;

/// Sent by the server to send data of a structure to the client in response to a
/// StructureTemplateDataRequest packet.
#[derive(Debug, Clone)]
pub struct StructureTemplateDataResponse {
    /// The name of the structure that was requested. This is the name used to export the structure
    /// to a file.
    pub structure_name: String,
    /// Specifies if a structure template was found by the StructureName that was sent in a
    /// StructureTemplateDataRequest packet.
    pub success: bool,
    /// The data of the structure template.
    pub structure_template: Value,
    /// The response type of the packet. This depends on the RequestType field sent in the
    /// StructureTemplateDataRequest packet.
    pub response_type: StructureTemplateDataRequestType,
}

impl PacketType for StructureTemplateDataResponse {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.structure_name.as_str());
        writer.bool(self.success);
        if self.success {
            writer.nbt(&self.structure_template, NetworkLittleEndian);
        }
        writer.u8(self.response_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        let structure_name = reader.string();
        let success = reader.bool();
        Self {
            structure_name,
            success,
            structure_template: if success { reader.nbt(NetworkLittleEndian) } else { Value::default() },
            response_type: StructureTemplateDataRequestType::from_u8(reader.u8()).unwrap(),
        }
    }
}
