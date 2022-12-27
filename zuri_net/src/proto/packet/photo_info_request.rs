use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct PhotoInfoRequest {
    pub photo_id: i64,
}

impl PacketType for PhotoInfoRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.photo_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            photo_id: reader.var_i64(),
        }
    }
}
