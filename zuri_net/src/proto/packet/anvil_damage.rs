use glam::IVec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct AnvilDamage {
    pub damage: u8,
    pub anvil_position: IVec3,
}

impl PacketType for AnvilDamage {
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
