use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SetDefaultGameType {
    pub game_type: i32,
}

impl Packet for SetDefaultGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: reader.var_i32(),
        }
    }
}
