use bytes::Bytes;

use zuri_net_derive::proto;

use crate::proto::ints::VarI64;

/// Sent by the server to the client. It does not seem to do anything when sent to the normal client
/// in 1.16.
#[proto]
#[derive(Debug, Clone)]
pub struct DebugInfo {
    /// The unique ID of the player that the packet is sent to.
    pub player_unique_id: VarI64,
    /// The debug data.
    pub data: Bytes,
}
