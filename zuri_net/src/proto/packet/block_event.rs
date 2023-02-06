use zuri_net_derive::packet;
use crate::proto::ints::VarI32;

use crate::proto::io::UBlockPos;

/// Sent by the server to initiate a certain event that has to do with blocks in specific, for
/// example opening chests.
#[packet]
#[derive(Debug, Clone)]
pub struct BlockEvent {
    /// The position of the block that an event occurred at.
    pub position: UBlockPos,
    /// The type of the block event. The event type decides the way the event data that follows is
    /// used.
    pub event_type: BlockEventType,
    /// Holds event type specific data. For chests, for example, opening the chest means the data
    /// must hold one, whereas closing it should hold zero.
    pub event_data: VarI32,
}

#[packet(VarI32)]
#[derive(Debug, Clone)]
pub enum BlockEventType {
    None,
    ChangeChestState,
}
