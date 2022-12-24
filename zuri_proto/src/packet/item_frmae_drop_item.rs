#[derive(Debug)]
pub struct ItemFrameDropItem {
    pub position: BlockPos,
}

impl Packet for ItemFrameDropItem {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
        }
    }
}
