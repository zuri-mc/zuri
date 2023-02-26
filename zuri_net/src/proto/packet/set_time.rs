use zuri_net_derive::proto;

use crate::proto::ints::VarI32;

/// Sent by the server to update the current time client-side. The client actually advances time
/// client-side by itself, so this packet does not need to be sent each tick. It is a means of
/// synchronising time between server and client.
#[proto]
#[derive(Debug, Clone)]
pub struct SetTime {
    /// The current time. The time is not limited to 24000 (time of day), but continues progressing
    /// after that.
    pub time: VarI32,
}
