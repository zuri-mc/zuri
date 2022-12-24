#[derive(Debug)]
pub struct BlockActorData {
    pub position: BlockPos,
    // pub nbt_data: dyn Any, // TODO: NBT
}

impl Packet for BlockActorData {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        // TODO: NBT (nbt_data)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            // nbt_data: {
            //     // TODO: NBT
            // },
        }
    }
}
