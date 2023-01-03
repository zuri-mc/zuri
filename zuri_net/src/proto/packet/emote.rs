use num_derive::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EmoteFlag {
    ServerSide
}

impl EmoteFlag {
    pub fn flag(&self) -> u8 {
        1 << (*self as u8)
    }
}

/// Sent by both the server and the client. When the client sends an emote, it sends this packet to
/// the server, after which the server will broadcast the packet to other players online.
#[derive(Debug, Clone)]
pub struct Emote {
    /// The entity that sent the emote. When a player sends this packet, it has this field set as
    /// its own entity runtime ID.
    pub entity_runtime_id: u64,
    /// The ID of the emote to send.
    pub emote_id: String,
    /// A combination of flags that change the way the Emote packet operates. When the server sends
    /// this packet to other players, the server side emote flag must be present.
    pub flags: u8,
}

impl PacketType for Emote {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.string(self.emote_id.as_str());
        writer.u8(self.flags);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            emote_id: reader.string(),
            flags: reader.u8(),
        }
    }
}
