use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct UpdateSubChunkBlocks {
    pub position: BlockPos,
    pub blocks: Vec<BlockChangeEntry>,
    pub extra: Vec<BlockChangeEntry>,
}

impl Packet for UpdateSubChunkBlocks {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.var_u32(self.blocks.len() as u32);
        self.blocks.iter().for_each(|entry| entry.write(writer));
        writer.var_u32(self.extra.len() as u32);
        self.extra.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            blocks: (0..reader.var_u32()).map(|_| BlockChangeEntry::read(reader)).collect(),
            extra: (0..reader.var_u32()).map(|_| BlockChangeEntry::read(reader)).collect(),
        }
    }
}
