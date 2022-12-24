use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::structure::StructureTemplateDataRequestType;

#[derive(Debug)]
pub struct StructureTemplateDataResponse {
    pub structure_name: String,
    pub success: bool,
    //pub structure_template: dyn Any,
    // TODO: NBT
    pub response_type: StructureTemplateDataRequestType,
}

impl Packet for StructureTemplateDataResponse {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.structure_name.as_str());
        writer.bool(self.success);
        if self.success {
            // TODO: NBT (structure_template)
        }
        writer.u8(self.response_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        let structure_name = reader.string();
        let success = reader.bool();
        Self {
            structure_name,
            success,
            // TODO: NBT (structure_template) if success
            response_type: StructureTemplateDataRequestType::from_u8(reader.u8()).unwrap(),
        }
    }
}
