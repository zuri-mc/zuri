use zuri_net_derive::packet;

use crate::proto::io::UBlockPos;

/// Sent by the client when it takes an item out of an item frame.
#[packet]
#[derive(Debug, Clone)]
pub struct ItemFrameDropItem {
    /// The position of the item frame that had its item dropped. There must be a 'block entity'
    /// present at this position.
    pub position: UBlockPos,
}
