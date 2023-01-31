use zuri_net_derive::packet;

/// Sent by the server to show the XBOX Live profile of one player to another.
#[packet]
#[derive(Debug, Clone)]
pub struct ShowProfile {
    /// The XBOX Live User ID of the player whose profile should be shown to the player. If it is
    /// not a valid XUID, the client ignores the packet.
    pub xuid: String,
}
