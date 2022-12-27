use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct CompletedUsingItem {
    pub used_item_id: i16,
    pub use_method: i32,
}

impl PacketType for CompletedUsingItem {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.used_item_id);
        writer.i32(self.use_method);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            used_item_id: reader.i16(),
            use_method: reader.i32(),
        }
    }
}
