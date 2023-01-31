use glam::IVec3;

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to update the server on which page was opened in a book on a lectern, or if
/// the book should be removed from it.
#[derive(Debug, Clone)]
pub struct LecternUpdate {
    /// The page number in the book that was opened by the player on the lectern.
    pub page: u8,
    /// The number of pages that the book opened in the lectern has.
    pub page_count: u8,
    /// The position of the lectern that was updated. If no lectern is at the block position, the
    /// packet should be ignored.
    pub position: IVec3,
    /// Specifies if the book currently set on display in the lectern should be dropped server-side.
    pub drop_book: bool,
}

impl PacketType for LecternUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.page);
        writer.u8(self.page_count);
        writer.u_block_pos(self.position);
        writer.bool(self.drop_book);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            page: reader.u8(),
            page_count: reader.u8(),
            position: reader.u_block_pos(),
            drop_book: reader.bool(),
        }
    }
}
