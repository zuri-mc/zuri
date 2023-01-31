use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::GameType;

/// Sent by the server to update the game type of a player.
#[derive(Debug, Clone)]
pub struct SetPlayerGameType {
    /// The new game type of the player. Some of these game types require additional flags to be set
    /// in an UpdateAbilities packet for the game mode to obtain its full functionality.
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
