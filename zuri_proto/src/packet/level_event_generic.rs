use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct LevelEventGeneric {
    pub event_id: i32,
    pub serialised_event_data: Bytes,
}

impl Packet for LevelEventGeneric {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.event_id);
        writer.byte_slice(&self.serialised_event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_id: reader.var_i32(),
            serialised_event_data: reader.byte_slice(),
        }
    }
}
