use bytes::Bytes;

use zuri_net_derive::proto;

/// Sent by the server to the client to complete the key exchange in order to initialise encryption
/// on client and server side. It is followed up by a ClientToServerHandshake packet.
#[proto]
#[derive(Debug, Clone)]
pub struct ServerToClientHandshake {
    /// A raw JWT token containing data such as the public key from the server, the algorithm used
    /// and the server's token. It is used for the client to produce a shared secret.
    pub jwt: Bytes,
}
