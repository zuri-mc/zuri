use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct DebugInfo {
    pub player_unique_id: i64,
    pub data: Bytes,
}

impl Packet for DebugInfo {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.player_unique_id);
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_unique_id: reader.var_i64(),
            data: reader.byte_slice(),
        }
    }
}
