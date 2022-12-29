use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub player_unique_id: i64,
    pub data: Bytes,
}

impl PacketType for DebugInfo {
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
