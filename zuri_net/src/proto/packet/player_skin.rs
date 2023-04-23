use uuid::Uuid;
use zuri_net_derive::proto;

use crate::proto::types::skin::Skin;

/// Sent by the client to the server when it updates its own skin using the in-game skin picker. It
/// is relayed by the server, or sent if the server changes the skin of a player on its own accord.
/// Note that the packet can only be sent for players that are in the player list.
#[proto]
#[derive(Debug, Clone)]
pub struct PlayerSkin {
    /// The UUID of the player as sent in the Login packet when the client joined the server. It
    /// must match this UUID exactly for the skin to show up on the player.
    pub uuid: Uuid,
    /// The new skin to be applied on the player with the UUID in the field above. The skin,
    /// including its animations, will be shown after sending it.
    pub skin: Skin,
    /// No longer has a function. The field can be left empty at all times.
    pub new_skin_name: String,
    /// No longer has a function. The field can be left empty at all times.
    pub old_skin_name: String,
    #[overwrite(skin.trusted)]
    pub trusted: bool,
}
