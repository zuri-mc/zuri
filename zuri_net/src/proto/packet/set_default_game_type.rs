use zuri_net_derive::proto;

use crate::proto::types::world::GameType;

/// Sent by the client when it toggles the default game type in the settings UI, and is sent by the
/// server when it actually changes the default game type, resulting in the toggle being changed in
/// the settings UI.
#[proto]
#[derive(Debug, Clone)]
pub struct SetDefaultGameType {
    /// The new game type that is set. When sent by the client, this is the requested new default
    /// game type.
    pub game_type: GameType,
}
