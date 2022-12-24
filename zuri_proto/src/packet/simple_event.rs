use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SimpleEvent {
    pub event_type: i16,
}

impl Packet for SimpleEvent {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.event_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_type: reader.i16(),
        }
    }
}
