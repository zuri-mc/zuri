use uuid::Uuid;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::types::skin::Skin;
use crate::io::{Reader, Writer};
use crate::types::player::PlayerActionType;

#[derive(Debug)]
pub struct PlayerList {
    pub action_type: PlayerActionType,
    pub entries: Vec<PlayerListEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PlayerListAction {
    Add,
    Remove,
}

#[derive(Debug, Default)]
pub struct PlayerListEntry {
    pub uuid: Uuid,
    pub entity_unique_id: i64,
    pub username: String,
    pub xuid: String,
    pub platform_chat_id: String,
    pub build_platform: i32,
    pub skin: Skin,
    pub teacher: bool,
    pub host: bool,
}

impl Packet for PlayerList {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| PlayerListEntry::read(reader, action_type)).collect(),
        }
    }
}

impl PlayerListEntry {
    pub fn write(&self, writer: &mut Writer, action: PlayerListAction) {
        writer.uuid(self.uuid);
        if action == PlayerListAction::Add {
            writer.var_i64(self.entity_unique_id);
            writer.string(self.username.as_str());
            writer.string(self.xuid.as_str());
            writer.string(self.platform_chat_id.as_str());
            writer.i32(self.build_platform);
            self.skin.write(writer);
            writer.bool(self.teacher);
            writer.bool(self.host);
        }
    }

    pub fn read(reader: &mut Reader, action: PlayerListAction) -> Self {
        let mut entry = Self {
            uuid: reader.uuid(),
            ..Default::default()
        };
        if action == PlayerListAction::Add {
            entry.entity_unique_id = reader.var_i64();
            entry.username = reader.string();
            entry.xuid = reader.string();
            entry.platform_chat_id = reader.string();
            entry.build_platform = reader.i32();
            entry.skin = Skin::read(reader);
            entry.teacher = reader.bool();
            entry.host = reader.bool();
        }

        entry
    }
}
