use glam::IVec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client when it takes an item out of an item frame.
#[derive(Debug, Clone)]
pub struct ItemFrameDropItem {
    /// The position of the item frame that had its item dropped. There must be a 'block entity'
    /// present at this position.
    pub position: IVec3,
}

impl PacketType for ItemFrameDropItem {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { position: reader.u_block_pos() }
    }
}
