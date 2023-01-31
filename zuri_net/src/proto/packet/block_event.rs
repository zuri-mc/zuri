use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to initiate a certain event that has to do with blocks in specific, for
/// example opening chests.
#[derive(Debug, Clone)]
pub struct BlockEvent {
    /// The position of the block that an event occurred at.
    pub position: IVec3,
    /// The type of the block event. The event type decides the way the event data that follows is
    /// used.
    pub event_type: BlockEventType,
    /// Holds event type specific data. For chests, for example, opening the chest means the data
    /// must hold one, whereas closing it should hold zero.
    pub event_data: i32,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum BlockEventType {
    None,
    ChangeChestState,
}

impl PacketType for BlockEvent {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_i32(self.event_type.to_i32().unwrap());
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            event_type: BlockEventType::from_i32(reader.var_i32()).unwrap(),
            event_data: reader.var_i32(),
        }
    }
}
