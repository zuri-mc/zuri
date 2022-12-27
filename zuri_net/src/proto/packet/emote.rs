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

#[derive(Debug, Clone)]
pub struct Emote {
    pub entity_runtime_id: u64,
    pub emote_id: String,
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
