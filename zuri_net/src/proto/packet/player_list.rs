use bevy::ecs::schedule::IntoRunCriteria;
use uuid::Uuid;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::types::skin::Skin;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::device::Device;

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PlayerListAction {
    Add,
    Remove,
}

/// Sent by the server to update the client-side player list in the in-game menu screen. It shows
/// the icon of each player if the correct XUID is written in the packet. Sending the PlayerList
/// packet is obligatory when sending an AddPlayer packet. The added player will not show up to a
/// client if it has not been added to the player list, because several properties of the player are
/// obtained from the player list, such as the skin.
#[derive(Debug, Clone)]
pub struct PlayerList {
    /// The action to execute upon the player list. The entries that follow specify which entries
    /// are added or removed from the player list.
    pub action_type: PlayerListAction,
    /// A list of all player list entries that should be added/removed from the player list,
    /// depending on the `action_type` set.
    pub entries: Vec<PlayerListEntry>,
}

impl PacketType for PlayerList {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|e| e.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = PlayerListAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| PlayerListEntry::read(reader, action_type)).collect(),
        }
    }
}

/// An entry found in the PlayerList packet. It represents a single player using the UUID found in
/// the entry, and contains several properties such as the skin.
#[derive(Debug, Clone)]
pub struct PlayerListEntry {
    /// The UUID of the player as sent in the Login packet when the client joined the server. It
    /// must match this UUID exactly for the correct XBOX Live icon to show up in the list.
    pub uuid: Uuid,
    /// The unique entity ID of the player. This ID typically stays consistent during the lifetime
    /// of a world, but servers often send the runtime ID for this.
    pub entity_unique_id: i64,
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

impl PlayerListEntry {
    pub fn write(&self, writer: &mut Writer, action: PlayerListAction) {
        writer.uuid(self.uuid);
        if action == PlayerListAction::Add {
            writer.var_i64(self.entity_unique_id);
            writer.string(self.username.as_str());
            writer.string(self.xuid.as_str());
            writer.string(self.platform_chat_id.as_str());
            writer.i32(self.build_platform.to_i32().unwrap());
            self.skin.write(writer);
            writer.bool(self.teacher);
            writer.bool(self.host);
        }
    }

    pub fn read(reader: &mut Reader, action: PlayerListAction) -> Self {
        let mut entry = Self {
            uuid: reader.uuid(),
            entity_unique_id: 0,
            username: "".to_string(),
            xuid: "".to_string(),
            platform_chat_id: "".to_string(),
            build_platform: Device::None,
            skin: Skin::default(),
            teacher: false,
            host: false,
        };
        if action == PlayerListAction::Add {
            entry.entity_unique_id = reader.var_i64();
            entry.username = reader.string();
            entry.xuid = reader.string();
            entry.platform_chat_id = reader.string();
            entry.build_platform = Device::from_i32(reader.i32()).unwrap();
            entry.skin = Skin::read(reader);
            entry.teacher = reader.bool();
            entry.host = reader.bool();
        }

        entry
    }
}
