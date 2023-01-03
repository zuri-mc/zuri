use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to request network settings, such as compression, from the server.
#[derive(Debug, Clone)]
pub struct RequestNetworkSettings {
    /// The protocol version of the player. The player is disconnected if the protocol is
    /// incompatible with the protocol of the server.
    pub client_protocol: i32,
}

impl PacketType for RequestNetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.client_protocol);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { client_protocol: reader.i32_be() }
    }
}
