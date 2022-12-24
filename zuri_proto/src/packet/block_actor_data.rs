use glam::IVec3;
use zuri_nbt::{Value, encoding::NetworkLittleEndian};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct BlockActorData {
    pub position: IVec3,
    pub nbt_data: Value,
}

impl Packet for BlockActorData {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.nbt(&self.payload, NetworkLittleEndian);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            nbt_data: reader.nbt(NetworkLittleEndian),
        }
    }
}
