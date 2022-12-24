use glam::IVec3;
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct LabTable {
    pub action_type: u8,
    pub position: IVec3,
    pub reaction_type: u8,
}

impl Packet for LabTable {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type);
        writer.block_pos(self.position);
        writer.u8(self.reaction_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.u8(),
            position: reader.block_pos(),
            reaction_type: reader.u8(),
        }
    }
}
