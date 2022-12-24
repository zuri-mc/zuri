use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct Emote {
    pub entity_runtime_id: u64,
    pub emote_id: String,
    pub flags: u8,
}

impl Packet for Emote {
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
