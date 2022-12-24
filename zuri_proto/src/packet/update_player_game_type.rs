use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct UpdatePlayerGameType {
    pub game_type: i32,
    pub player_unique_id: i64,
}

impl PacketType for UpdatePlayerGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type);
        writer.var_i64(self.player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: reader.var_i32(),
            player_unique_id: reader.var_i64(),
        }
    }
}
