use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct LabTable {
    pub action_type: LabTableAction,
    pub position: IVec3,
    pub reaction_type: u8,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum LabTableAction {
    Combine,
    React,
    Reset,
}

impl Packet for LabTable {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());
        writer.block_pos(self.position);
        writer.u8(self.reaction_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: LabTableAction::from_u8(reader.u8()).unwrap(),
            position: reader.block_pos(),
            reaction_type: reader.u8(),
        }
    }
}
