use zuri_net_derive::packet;

use crate::proto::types::world::Difficulty;

/// Sent by the server to update the client-side difficulty of the client. The actual effect of this
/// packet on the client isn't very significant, as the difficulty is handled server-side.
#[packet]
#[derive(Debug, Clone)]
pub struct SetDifficulty {
    /// The new difficulty that the world has.
    pub difficulty: Difficulty,
}
