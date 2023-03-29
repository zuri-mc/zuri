use crate::proto::ints::VarI64;
use zuri_net_derive::proto;

use crate::proto::types::world::GameType;

/// Sent by the server to change the game mode of a player. It is functionally identical to the
/// SetPlayerGameType packet.
#[proto]
#[derive(Debug, Clone)]
pub struct UpdatePlayerGameType {
    /// The new game type of the player. Some of these game types require additional flags to be set
    /// in an UpdateAbilities packet for the game mode to obtain its full functionality.
    pub game_type: GameType,
    /// The entity unique ID of the player that should have its game mode updated. If this packet is
    /// sent to other clients with the player unique ID of another player, nothing happens.
    pub player_unique_id: VarI64,
}
