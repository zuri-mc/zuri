use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

/// Sent by the server to show the Minecraft credits screen to the client. It is typically sent when
/// the player beats the ender dragon and leaves the End.
#[proto]
#[derive(Debug, Clone)]
pub struct ShowCredits {
    /// The entity runtime ID of the player to show the credits to. It's not clear why this field is
    /// actually here in the first place.
    pub player_runtime_id: u64,
    /// The status type of the credits. It either starts or stops the credits.
    pub status_type: ShowCreditsStatus,
}

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum ShowCreditsStatus {
    Start,
    End,
}
