use zuri_net_derive::proto;

use crate::proto::types::world::GameType;

/// Sent by the server to update the game type of a player.
#[proto]
#[derive(Debug, Clone)]
pub struct SetPlayerGameType {
    /// The new game type of the player. Some of these game types require additional flags to be set
    /// in an UpdateAbilities packet for the game mode to obtain its full functionality.
    pub game_type: GameType,
}
