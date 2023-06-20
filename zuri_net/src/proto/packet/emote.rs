use num_derive::{FromPrimitive, ToPrimitive};

use zuri_net_derive::proto;

use crate::proto::ints::VarU64;

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EmoteFlag {
    ServerSide,
}

impl EmoteFlag {
    pub fn flag(&self) -> u8 {
        1 << (*self as u8)
    }
}

/// Sent by both the server and the client. When the client sends an emote, it sends this packet to
/// the server, after which the server will broadcast the packet to other players online.
#[proto]
#[derive(Debug, Clone)]
pub struct Emote {
    /// The entity that sent the emote. When a player sends this packet, it has this field set as
    /// its own entity runtime ID.
    pub entity_runtime_id: VarU64,
    /// The ID of the emote to send.
    pub emote_id: String,
    /// A combination of flags that change the way the Emote packet operates. When the server sends
    /// this packet to other players, the server side emote flag must be present.
    pub flags: u8,
}
