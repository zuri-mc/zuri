use uuid::Uuid;
use zuri_net_derive::proto;

use crate::proto::ints::{VarI64, VarU32};
use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::types::device::Device;
use crate::proto::types::skin::Skin;

#[proto(u8)]
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
#[proto]
#[derive(Debug, Clone)]
pub struct PlayerList {
    /// The action to execute upon the player list. The entries that are contained specify which
    /// entries are added or removed from the player list.
    pub action_type: PlayerListAction,
}

#[proto]
#[derive(Clone, Debug)]
pub struct PlayerListRemove {
    /// A list of UUIDs to remove.
    #[len_type(VarU32)]
    pub uuids: Vec<Uuid>,
}

#[derive(Clone, Debug)]
pub struct PlayerListAdd {
    pub entries: Vec<PlayerListEntry>,
}

impl Writable for PlayerListAdd {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.entries.len() as u32);
        for entry in &self.entries {
            entry.write(writer);
        }
        for entry in &self.entries {
            writer.bool(entry.skin.trusted);
        }
    }
}

impl Readable<PlayerListAdd> for PlayerListAdd {
    fn read(reader: &mut Reader) -> PlayerListAdd {
        let entry_count = reader.var_u32();
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entries.push(PlayerListEntry::read(reader));
        }
        for i in 0..entry_count {
            entries[i as usize].skin.trusted = reader.bool();
        }
        PlayerListAdd { entries }
    }
}

/// An entry found in the PlayerList packet. It represents a single player using the UUID found in
/// the entry, and contains several properties such as the skin.
#[proto]
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
    pub build_platform: Device,
    /// The skin of the player that should be added to the player list. Once sent here, it will not
    /// have to be sent again.
    pub skin: Skin,
    /// Minecraft: Education Edition field. It specifies if the player to be added to the player
    /// list is a teacher.
    pub teacher: bool,
    /// Specifies if the player that is added to the player list is the host of the game.
    pub host: bool,
}
