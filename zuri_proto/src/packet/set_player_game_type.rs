use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};
use crate::types::world::GameType;

#[derive(Debug)]
pub struct SetPlayerGameType {
    pub game_type: GameType,
}

impl PacketType for SetPlayerGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { game_type: GameType::from_i32(reader.var_i32()).unwrap() }
    }
}
