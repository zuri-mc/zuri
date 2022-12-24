use bytes::Bytes;
use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server to the client to complete the key exchange in order to initialise encryption on client and server
/// side. It is followed up by a ClientToServerHandshake packet from the client.
#[derive(Debug)]
pub struct ServerToClientHandshake {
    /// A raw JWT token containing data such as the public key from the server, the algorithm used and the server's
    /// token. It is used for the client to produce a shared secret.
    pub jwt: Bytes,
}

impl Packet for ServerToClientHandshake {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.jwt);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { jwt: reader.byte_slice() }
    }
}
