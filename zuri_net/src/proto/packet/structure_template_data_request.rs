use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::structure::{StructureSettings, StructureTemplateDataRequestType};

/// Sent by the client to request data of a structure.
#[derive(Debug, Clone)]
pub struct StructureTemplateDataRequest {
    /// Name of the structure that was set in the structure block's UI. This is the name used to
    /// export the structure to a file.
    pub structure_name: String,
    /// The position of the structure block that has its template data requested.
    pub position: IVec3,
    /// Settings that should be used for exporting the structure. These settings are identical to
    /// the last sent in the StructureBlockUpdate packet by the client.
    pub settings: StructureSettings,
    /// The type of template data request that the player sent.
    pub request_type: StructureTemplateDataRequestType,
}

impl PacketType for StructureTemplateDataRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.structure_name.as_str());
        writer.u_block_pos(self.position);
        self.settings.write(writer);
        writer.u8(self.request_type.to_u8().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            structure_name: reader.string(),
            position: reader.u_block_pos(),
            settings: StructureSettings::read(reader),
            request_type: StructureTemplateDataRequestType::from_u8(reader.u8()).unwrap(),
        }
    }
}
