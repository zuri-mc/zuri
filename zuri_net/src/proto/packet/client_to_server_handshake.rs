use zuri_net_derive::proto;

/// Sent by the client in response to a ServerToClientHandshake packet sent by the server. It is the
/// first encrypted packet in the login handshake and serves as a confirmation that encryption is
/// correctly initialised client side.
#[proto]
#[derive(Debug, Clone)]
pub struct ClientToServerHandshake;
