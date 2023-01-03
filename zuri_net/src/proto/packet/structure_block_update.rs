use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::structure::{StructureBlockType, StructureRedstoneSaveMode, StructureSettings};

/// Sent by the client when it updates a structure block using the in-game UI. The data it contains
/// depends on the type of structure block that it is. In Minecraft Bedrock Edition v1.11, there is
/// only the `Export `structure block type, but in v1.13 the ones present in Java Edition will,
/// according to the wiki, be added too.
#[derive(Debug, Clone)]
pub struct StructureBlockUpdate {
    /// The position of the structure block that is updated.
    pub position: IVec3,
    /// The name of the structure that was set in the structure block's UI. This is the name used to
    /// export the structure to a file.
    pub structure_name: String,
    /// The name of a function to run, usually used during natural generation. A description can be
    /// found here: https://minecraft.gamepedia.com/Structure_Block#Data.
    pub data_field: String,
    /// Specifies if the 'Include Players' toggle has been enabled, meaning players are also
    /// exported by the structure block.
    pub include_players: bool,
    /// Specifies if the structure block should have its bounds outlined. A thin line will surround
    /// the bounds of the structure if set to true.
    pub show_bounding_box: bool,
    /// The type of the structure block updated.
    pub structure_block_type: StructureBlockType,
    /// Settings that should be used for exporting the structure. These settings are identical to
    /// the last sent in the StructureBlockUpdate packet by the client.
    pub settings: StructureSettings,
    /// The mode that should be used to save the structure when used with redstone. In Java Edition,
    /// this is always stored in memory, but in Bedrock Edition it can be stored either to disk or
    /// memory.
    pub redstone_save_mode: StructureRedstoneSaveMode,
    /// Specifies if the structure block should be triggered immediately after this packet reaches
    /// the server.
    pub should_trigger: bool,
    /// Specifies if non-air blocks replace water or combine with water.
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
