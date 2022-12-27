use num_traits::{FromPrimitive, ToPrimitive};
use zuri_nbt::{Value, encoding::NetworkLittleEndian};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::structure::StructureTemplateDataRequestType;

#[derive(Debug)]
pub struct StructureTemplateDataResponse {
    pub structure_name: String,
    pub success: bool,
    pub structure_template: Value,
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
