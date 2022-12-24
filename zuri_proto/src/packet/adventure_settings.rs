use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::world::PermissionLevel;
use crate::types::command::CommandPermissionLevel;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AdventureFlag {
    WorldImmutable,
    NoPvM,
    NoPvP,
    Unused,
    ShowNameTags,
    AutoJump,
    AllowFlight,
    NoClip,
    WorldBuilder,
    Flying,
    Muted,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ActionPermission {
    Mine,
    DoorsAndSwitches,
    OpenContainers,
    AttackPlayers,
    AttackMobs,
    OperatorCommands,
    Teleport,
    Build,
    Default,
}

#[derive(Debug)]
pub struct AdventureSettings {
    pub flags: u32,
    pub command_permission_level: CommandPermissionLevel,
    pub action_permissions: u32,
    pub permission_level: PermissionLevel,
    pub custom_stored_permissions: u32,
    pub player_unique_id: i64,
}

impl Packet for AdventureSettings {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.flags);
        writer.var_u32(self.command_permission_level.to_u32().unwrap());
        writer.var_u32(self.action_permissions);
        writer.var_u32(self.permission_level.to_u32().unwrap());
        writer.var_u32(self.custom_stored_permissions);
        writer.i64(self.player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            flags: reader.var_u32(),
            command_permission_level: CommandPermissionLevel::from_u32(reader.var_u32()).unwrap(),
            action_permissions: reader.var_u32(),
            permission_level: PermissionLevel::from_u32(reader.var_u32()).unwrap(),
            custom_stored_permissions: reader.var_u32(),
            player_unique_id: reader.i64(),
        }
    }
}
