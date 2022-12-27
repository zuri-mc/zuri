use glam::IVec3;

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone)]
pub struct LecternUpdate {
    pub page: u8,
    pub page_count: u8,
    pub position: IVec3,
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
