use uuid::Uuid;

use zuri_net_derive::proto;

use crate::proto::ints::{VarU32, VarU64};

/// Sent by the client every time it joins the server and when it equips new emotes. It may be used
/// by the server to find out which emotes the client has available. If the player has no emotes
/// equipped, this packet is not sent. Under certain circumstances, this packet is also sent from
/// the server to the client, but I was unable to find when this is done.
#[proto]
#[derive(Debug, Clone)]
pub struct EmoteList {
    /// The runtime ID of the player that owns the emote pieces below. If sent by the client, this
    /// player runtime ID is always that of the player itself.
    pub player_runtime_id: VarU64,
    /// A list of emote pieces that the player with the runtime ID above has.
    #[len_type(VarU32)]
    pub emote_pieces: Vec<Uuid>,
}
