use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::GameType;

/// Sent by the server to change the game mode of a player. It is functionally identical to the
/// SetPlayerGameType packet.
#[derive(Debug, Clone)]
pub struct UpdatePlayerGameType {
    /// The new game type of the player. Some of these game types require additional flags to be set
    /// in an UpdateAbilities packet for the game mode to obtain its full functionality.
    pub game_type: GameType,
    /// The entity unique ID of the player that should have its game mode updated. If this packet is
    /// sent to other clients with the player unique ID of another player, nothing happens.
    pub player_unique_id: i64,
}

impl PacketType for UpdatePlayerGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type.to_i32().unwrap());
        writer.var_i64(self.player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: GameType::from_i32(reader.var_i32()).unwrap(),
            player_unique_id: reader.var_i64(),
        }
    }
}
