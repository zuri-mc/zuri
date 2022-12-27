use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client to complete the key exchange in order to initialise encryption on client and server
/// side. It is followed up by a ClientToServerHandshake packet from the client.
#[derive(Debug, Clone)]
pub struct ServerToClientHandshake {
    /// A raw JWT token containing data such as the public key from the server, the algorithm used and the server's
    /// token. It is used for the client to produce a shared secret.
    pub jwt: Bytes,
}

impl PacketType for ServerToClientHandshake {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.jwt);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { jwt: reader.byte_slice() }
    }
}
