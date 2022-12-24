use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the client to request data of a structure.
#[derive(Debug)]
pub struct StructureTemplateDataRequest {
    /// Name of the structure that was set in the structure block's UI. This is the name used to export the structure
    /// to a file.
    pub structure_name: String,
    pub position: BlockPos,
    pub settings: StructureSettings,
    pub request_type: StructureTemplateDataRequestType,
}

impl Packet for StructureTemplateDataRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.structure_name.as_str());
        writer.u_block_pos(self.position);
        self.settings.write(writer);
        writer.u8(num::ToPrimitive::to_u8(&self.request_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            structure_name: reader.string(),
            position: reader.u_block_pos(),
            settings: StructureSettings::read(reader),
            request_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}
