use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client. It does not seem to do anything when sent to the normal client
/// in 1.16.
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// The unique ID of the player that the packet is sent to.
    pub player_unique_id: i64,
    /// The debug data.
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
