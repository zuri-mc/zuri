use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum CommandBlockMode {
    None = -1,
    Impulse,
    Repeating,
    Chain,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandBlockVariant {
    Block,
    Minecart,
}

/// Sent by the client to update a command block at a specific position. The command block may be
/// either a physical block or an entity.
#[derive(Debug, Clone)]
pub struct CommandBlockUpdate {
    /// Specifies the variant of the command block, as command blocks can be blocks, minecarts, and
    /// potentially other objects in the future.
    pub variant: CommandBlockVariant,
    /// The position of the command block updated. It is only set for block variants. Nothing
    /// happens if no command block is set at this position.
    pub position: IVec3,
    /// The mode of the command block. It is either impulse, chain or repeat. It is only set for
    /// block variants.
    pub mode: CommandBlockMode,
    /// Specifies if the command block needs to be powered by redstone to be activated. If false,
    /// the command block is always active. It is only set for block variants.
    pub needs_redstone: bool,
    /// Specifies the behaviour of the command block if the command block before it (the opposite
    /// side of the direction the arrow if facing) fails to execute. If set to false, it will
    /// activate at all times, whereas if set to true, it will activate only if the previous command
    /// block executed successfully. It is only set for block variants.
    pub conditional: bool,
    /// The runtime ID of the minecart entity carrying the command block that is updated. It is only
    /// set for minecart variants.
    pub minecart_entity_runtime_id: u64,
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

impl PacketType for CommandBlockUpdate {
    fn write(&self, writer: &mut Writer) {
        match self.variant {
            CommandBlockVariant::Block => {
                writer.bool(true);
                writer.u_block_pos(self.position);
                writer.var_u32(self.mode.to_u32().unwrap()); // todo
                writer.bool(self.needs_redstone);
                writer.bool(self.conditional);
            }
            CommandBlockVariant::Minecart => {
                writer.bool(false);
                writer.u64(self.minecart_entity_runtime_id);
            }
        }
        writer.string(self.command.as_str());
        writer.string(self.last_output.as_str());
        writer.string(self.name.as_str());
        writer.bool(self.should_track_output);
        writer.i32(self.tick_delay);
        writer.bool(self.execute_on_first_tick);
    }

    fn read(reader: &mut Reader) -> Self {
        let variant = if reader.bool() {
            CommandBlockVariant::Block
        } else {
            CommandBlockVariant::Minecart
        };
        Self {
            variant: variant.clone(),
            position: if variant == CommandBlockVariant::Block {
                reader.u_block_pos()
            } else {
                IVec3::default()
            },
            mode: if variant == CommandBlockVariant::Block {
                CommandBlockMode::from_u32(reader.var_u32()).unwrap()
            } else {
                CommandBlockMode::None
            },
            needs_redstone: if variant == CommandBlockVariant::Block {
                reader.bool()
            } else { false },
            conditional: if variant == CommandBlockVariant::Block {
                reader.bool()
            } else { false },
            minecart_entity_runtime_id: if variant == CommandBlockVariant::Minecart {
                reader.u64()
            } else { 0 },
            command: reader.string(),
            last_output: reader.string(),
            name: reader.string(),
            should_track_output: reader.bool(),
            tick_delay: reader.i32(),
            execute_on_first_tick: reader.bool(),
        }
    }
}
