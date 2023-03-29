use zuri_net_derive::proto;

use crate::proto::io::UBlockPos;
use crate::proto::types::structure::{
    StructureBlockType, StructureRedstoneSaveMode, StructureSettings,
};

/// Sent by the client when it updates a structure block using the in-game UI. The data it contains
/// depends on the type of structure block that it is. In Minecraft Bedrock Edition v1.11, there is
/// only the `Export `structure block type, but in v1.13 the ones present in Java Edition will,
/// according to the wiki, be added too.
#[proto]
#[derive(Debug, Clone)]
pub struct StructureBlockUpdate {
    /// The position of the structure block that is updated.
    pub position: UBlockPos,
    /// The name of the structure that was set in the structure block's UI. This is the name used to
    /// export the structure to a file.
    pub structure_name: String,
    /// The name of a function to run, usually used during natural generation. A description can be
    /// found here: <https://minecraft.gamepedia.com/Structure_Block#Data>.
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
