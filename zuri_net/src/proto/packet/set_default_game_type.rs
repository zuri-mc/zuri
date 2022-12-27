use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct SetDefaultGameType {
    pub game_type: i32,
}

impl PacketType for SetDefaultGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: reader.var_i32(),
        }
    }
}
