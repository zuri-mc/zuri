use zuri_net_derive::proto;

/// Sent by the server to transfer a player from the current server to another. Doing so will fully
/// disconnect the client, bring it back to the main menu and make it connect to the next server.
#[proto]
#[derive(Debug, Clone)]
pub struct Transfer {
    /// The address of the new server, which might be either a hostname or an actual IP address.
    pub address: String,
    /// The UDP port of the new server.
    pub port: u16,
}
