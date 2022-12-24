use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct CommandBlockUpdate {
    pub block: bool,
    pub position: BlockPos,
    pub mode: u32,
    pub needs_redstone: bool,
    pub conditional: bool,
    pub minecart_entity_runtime_id: u64,
    pub command: String,
    pub last_output: String,
    pub name: String,
    pub should_track_output: bool,
    pub tick_delay: i32,
    pub execute_on_first_tick: bool,
}

impl Packet for CommandBlockUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.block);
        if self.block {
            writer.u_block_pos(self.position);
            writer.var_u32(self.mode);
            writer.bool(self.needs_redstone);
            writer.bool(self.conditional);
        } else {
            writer.u64(self.minecart_entity_runtime_id);
        }
        writer.string(self.command.as_str());
        writer.string(self.last_output.as_str());
        writer.string(self.name.as_str());
        writer.bool(self.should_track_output);
        writer.i32(self.tick_delay);
        writer.bool(self.execute_on_first_tick);
    }

    fn read(reader: &mut Reader) -> Self {
        let block = reader.bool();
        Self {
            block,
            position: if block { reader.u_block_pos() } else { BlockPos::default() },
            mode: if block { reader.var_u32() } else { 0 },
            needs_redstone: if block { reader.bool() } else { false },
            conditional: if block { reader.bool() } else { false },
            minecart_entity_runtime_id: if !block { reader.u64() } else { 0 },
            command: reader.string(),
            last_output: reader.string(),
            name: reader.string(),
            should_track_output: reader.bool(),
            tick_delay: reader.i32(),
            execute_on_first_tick: reader.bool(),
        }
    }
}
