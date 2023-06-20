use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::io::UBlockPos;

/// Sent by the client to update a command block at a specific position. The command block may be
/// either a physical block or an entity.
#[proto]
#[derive(Debug, Clone)]
pub struct CommandBlockUpdate {
    /// Specifies the variant of the command block, as command blocks can be blocks, minecarts, and
    /// potentially other objects in the future.
    pub variant: CommandBlockVariant,
    /// The command currently entered in the command block. This is the command that is executed
    /// when the command block is activated.
    pub command: String,
    /// The output of the last command executed by the command block. It may be left empty to show
    /// simply no output at all, in combination with setting should track output to false.
    pub last_output: String,
    /// The name of the command block updated. If not empty, it will show this name hovering above
    /// the command block when hovering over the block with the cursor.
    pub name: String,
    /// Specifies if the command block tracks output. If set to false, the output box won't be shown
    /// within the command block.
    pub should_track_output: bool,
    /// The delay in ticks between executions of a command block, if it is a repeating command
    /// block.
    pub tick_delay: i32,
    /// Specifies if the command block should execute on the first tick, AKA as soon as the command
    /// block is enabled.
    pub execute_on_first_tick: bool,
}

#[proto(VarU32)]
#[derive(Debug, Clone, PartialEq)]
pub enum CommandBlockMode {
    Impulse,
    Repeating,
    Chain,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum CommandBlockVariant {
    Minecart(CommandBlockVariantMinecart),
    Block(CommandBlockVariantBlock),
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBlockVariantBlock {
    /// The position of the command block updated. Nothing happens if no command block is set at
    /// this position.
    pub position: UBlockPos,
    /// The mode of the command block. It is either impulse, chain or repeat.
    pub mode: CommandBlockMode,
    /// Specifies if the command block needs to be powered by redstone to be activated. If false,
    /// the command block is always active.
    pub needs_redstone: bool,
    /// Specifies the behaviour of the command block if the command block before it (the opposite
    /// side of the direction the arrow if facing) fails to execute. If set to false, it will
    /// activate at all times, whereas if set to true, it will activate only if the previous command
    /// block executed successfully.
    pub conditional: bool,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBlockVariantMinecart {
    /// The runtime ID of the minecart entity carrying the command block that is updated.
    pub minecart_entity_runtime_id: u64,
}
