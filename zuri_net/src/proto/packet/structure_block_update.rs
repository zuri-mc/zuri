use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::structure::{StructureBlockType, StructureRedstoneSaveMode, StructureSettings};

#[derive(Debug, Clone)]
pub struct StructureBlockUpdate {
    pub position: IVec3,
    pub structure_name: String,
    pub data_field: String,
    pub include_players: bool,
    pub show_bounding_box: bool,
    pub structure_block_type: StructureBlockType,
    pub settings: StructureSettings,
    pub redstone_save_mode: StructureRedstoneSaveMode,
    pub should_trigger: bool,
    pub waterlogged: bool,
}

impl PacketType for StructureBlockUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.string(self.structure_name.as_str());
        writer.string(self.data_field.as_str());
        writer.bool(self.include_players);
        writer.bool(self.show_bounding_box);
        writer.var_i32(self.structure_block_type.to_i32().unwrap());
        self.settings.write(writer);
        writer.var_i32(self.redstone_save_mode.to_i32().unwrap());
        writer.bool(self.should_trigger);
        writer.bool(self.waterlogged);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            structure_name: reader.string(),
            data_field: reader.string(),
            include_players: reader.bool(),
            show_bounding_box: reader.bool(),
            structure_block_type: StructureBlockType::from_i32(reader.var_i32()).unwrap(),
            settings: StructureSettings::read(reader),
            redstone_save_mode: StructureRedstoneSaveMode::from_i32(reader.var_i32()).unwrap(),
            should_trigger: reader.bool(),
            waterlogged: reader.bool(),
        }
    }
}
