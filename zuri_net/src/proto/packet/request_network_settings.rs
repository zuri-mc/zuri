use crate::proto::ints::I32BE;
use zuri_net_derive::proto;

/// Sent by the client to request network settings, such as compression, from the server.
#[proto]
#[derive(Debug, Clone)]
pub struct RequestNetworkSettings {
    /// The protocol version of the player. The player is disconnected if the protocol is
    /// incompatible with the protocol of the server.
    pub client_protocol: I32BE,
}
