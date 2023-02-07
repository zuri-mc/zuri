use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the server. It sets the health of the player it is sent to. The SetHealth packet should
/// no longer be used. Instead, the health attribute should be used so that the health and maximum
/// health may be changed directly.
#[proto]
#[derive(Debug, Clone)]
pub struct SetHealth {
    /// The new health of the player.
    pub health: VarI32,
}
