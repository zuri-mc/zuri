use uuid::Uuid;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client every time it joins the server and when it equips new emotes. It may be used
/// by the server to find out which emotes the client has available. If the player has no emotes
/// equipped, this packet is not sent. Under certain circumstances, this packet is also sent from
/// the server to the client, but I was unable to find when this is done.
#[derive(Debug, Clone)]
pub struct EmoteList {
    /// The runtime ID of the player that owns the emote pieces below. If sent by the client, this
    /// player runtime ID is always that of the player itself.
    pub player_runtime_id: u64,
    /// A list of emote pieces that the player with the runtime ID above has.
    pub emote_pieces: Vec<Uuid>,
}

impl PacketType for EmoteList {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.player_runtime_id);
        writer.var_u32(self.emote_pieces.len() as u32);
        self.emote_pieces.iter().for_each(|emote| writer.uuid(*emote));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_runtime_id: reader.var_u64(),
            emote_pieces: (0..reader.var_u32()).map(|_| reader.uuid()).collect(),
        }
    }
}
