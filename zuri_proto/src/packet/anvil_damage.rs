#[derive(Debug)]
pub struct AnvilDamage {
    pub damage: u8,
    pub anvil_position: BlockPos,
}

impl Packet for AnvilDamage {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.damage);
        writer.u_block_pos(self.anvil_position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            damage: reader.u8(),
            anvil_position: reader.u_block_pos(),
        }
    }
}
