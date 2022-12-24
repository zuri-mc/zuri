use bytes::Bytes;
use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent when the client initially tries to join the server. It is the first packet sent and contains information
/// specific to the player.
#[derive(Debug)]
pub struct Login {
    /// The protocol version of the player. The player is disconnected if the protocol is incompatible with the
    /// protocol of the server. It has been superseded by the protocol version sent in the RequestNetworkSettings
    /// packet, so this should no longer be used by the server.
    pub client_protocol: i32,
    /// A string containing information about the player and JWTs that may be used to verify if the player is connected
    /// to XBOX Live. The connection request also contains the necessary client public key to initiate encryption.
    pub connection_request: Bytes,
}

impl Packet for Login {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.client_protocol);
        writer.byte_slice(&self.connection_request);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_protocol: reader.i32_be(),
            connection_request: reader.byte_slice(),
        }
    }
}
