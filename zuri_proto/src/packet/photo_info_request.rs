use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct PhotoInfoRequest {
    pub photo_id: i64,
}

impl Packet for PhotoInfoRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.photo_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            photo_id: reader.var_i64(),
        }
    }
}
