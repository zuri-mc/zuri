use uuid::Uuid;
use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct EmoteList {
    pub player_runtime_id: u64,
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
