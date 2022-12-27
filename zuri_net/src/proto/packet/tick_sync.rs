use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client and the server to maintain a synchronized, server-authoritative tick between the client and the
/// server. The client sends this packet first, and the server should reply with another one of these packets, including
/// the response time.
#[derive(Debug)]
pub struct TickSync {
    /// The timestamp on which the client sent this packet to the server. The server should fill out that same value
    /// when replying. The client_request_timestamp is always zero.
    pub client_request_timestamp: i64,
    /// The timestamp on which the server received the packet sent by the client. When the packet is sent by the client,
    /// this value is zero. server_reception_timestamp is generally the current tick of the server. It isn't an actual
    /// timestamp, as the field implies.
    pub server_reception_timestamp: i64,
}

impl PacketType for TickSync {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.client_request_timestamp);
        writer.i64(self.server_reception_timestamp);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_request_timestamp: reader.i64(),
            server_reception_timestamp: reader.i64(),
        }
    }
}
