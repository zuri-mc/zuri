use uuid::Uuid;
use zuri_net_derive::packet;

use crate::proto::ints::{VarI64, VarU32};
use crate::proto::types::skin::Skin;

#[packet(u8)]
#[derive(Clone, Debug)]
pub enum PlayerListAction {
    Add(PlayerListAdd),
    Remove(PlayerListRemove),
}

/// Sent by the server to update the client-side player list in the in-game menu screen. It shows
/// the icon of each player if the correct XUID is written in the packet. Sending the PlayerList
/// packet is obligatory when sending an AddPlayer packet. The added player will not show up to a
/// client if it has not been added to the player list, because several properties of the player are
/// obtained from the player list, such as the skin.
#[packet]
#[derive(Debug, Clone)]
pub struct PlayerList {
    /// The action to execute upon the player list. The entries that are contained specify which
    /// entries are added or removed from the player list.
    pub action_type: PlayerListAction,
}

#[packet]
#[derive(Clone, Debug)]
pub struct PlayerListRemove {
    /// A list of UUIDs to remove.
    #[size_type(VarU32)]
    pub uuids: Vec<Uuid>,
}

#[packet]
#[derive(Clone, Debug)]
pub struct PlayerListAdd {
    #[size_type(VarU32)]
    pub uuids: Vec<PlayerListEntry>,
}

/// An entry found in the PlayerList packet. It represents a single player using the UUID found in
/// the entry, and contains several properties such as the skin.
#[packet]
#[derive(Debug, Clone)]
pub struct PlayerListEntry {
    /// The UUID of the player as sent in the Login packet when the client joined the server. It
    /// must match this UUID exactly for the correct XBOX Live icon to show up in the list.
    pub uuid: Uuid,
    /// The unique entity ID of the player. This ID typically stays consistent during the lifetime
    /// of a world, but servers often send the runtime ID for this.
    pub entity_unique_id: VarI64,
    /// The username that is shown in the player list of the player that obtains a PlayerList packet
    /// with this entry. It does not have to be the same as the actual username of the player.
    pub username: String,
    /// The XBOX Live user ID of the player, which will remain consistent as long as the player is
    /// logged in with the XBOX Live account.
    pub xuid: String,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo
    /// Switch). It is otherwise an empty string, and is used to decide which players are able to
    /// chat with each other.
    pub platform_chat_id: String,
    /// The platform of the player as sent by that player in the Login packet.
    pub build_platform: i32, // todo: allow for an unknown `Device` to be specified
    /// The skin of the player that should be added to the player list. Once sent here, it will not
    /// have to be sent again.
    pub skin: Skin,
    /// Minecraft: Education Edition field. It specifies if the player to be added to the player
    /// list is a teacher.
    pub teacher: bool,
    /// Specifies if the player that is added to the player list is the host of the game.
    pub host: bool,
    /// Specified whether the user's skin is trusted.
    pub trusted: bool, // todo: deduce this from the skin field
}
