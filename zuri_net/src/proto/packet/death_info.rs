use zuri_net_derive::proto;

use crate::proto::ints::VarU32;

/// Sent from the server to the client expected to be sent when a player dies. It contains messages
/// related to the player's death, which are shown on the death screen as of v1.19.10.
#[proto]
#[derive(Debug, Clone)]
pub struct DeathInfo {
    /// The cause of the player's death, such as "suffocation" or "suicide".
    pub cause: String,
    /// A list of death messages to be shown on the death screen.
    #[len_type(VarU32)]
    pub messages: Vec<String>,
}
