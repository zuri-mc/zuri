use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum LabTableAction {
    Combine,
    React,
    Reset,
}

/// Sent by the client to let the server know it started a chemical reaction in Education Edition,
/// and is sent by the server to other clients to show the effects. The packet is only functional if
/// Education features are enabled.
#[derive(Debug, Clone)]
pub struct LabTable {
    /// The type of the action that was executed. Typically, only combine is sent by the client,
    /// whereas react is sent by the server.
    pub action_type: LabTableAction,
    /// The position at which the lab table used was located.
    pub position: IVec3,
    /// The type of the reaction that took place as a result of the items put into the lab table.
    /// The reaction type can be either that of an item or a particle, depending on whatever the
    /// result was of the reaction.
    pub reaction_type: u8,
}

impl PacketType for LabTable {
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
