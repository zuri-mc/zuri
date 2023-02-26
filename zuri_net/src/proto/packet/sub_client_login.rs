use bytes::Bytes;

use zuri_net_derive::proto;

/// Sent when a sub-client joins the server while another client is already connected to it. The
/// packet is sent as a result of split-screen game play, and allows up to four players to play
/// using the same network connection. After an initial Login packet from the 'main' client, each
/// sub-client that connects sends a SubClientLogin to request their own login.
#[proto]
#[derive(Debug, Clone)]
pub struct SubClientLogin {
    /// A string containing information about the player and JWTs that may be used to verify if the
    /// player is connected to XBOX Live. The connection request also contains the necessary client
    /// public key to initiate encryption. The connection request in this packet is identical to the
    /// one found in the Login packet.
    pub connection_request: Bytes,
}
