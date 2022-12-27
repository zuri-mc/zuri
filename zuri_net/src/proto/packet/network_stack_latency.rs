use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct NetworkStackLatency {
    pub timestamp: i64,
    pub needs_response: bool,
}

impl PacketType for NetworkStackLatency {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.timestamp);
        writer.bool(self.needs_response);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            timestamp: reader.i64(),
            needs_response: reader.bool(),
        }
    }
}
