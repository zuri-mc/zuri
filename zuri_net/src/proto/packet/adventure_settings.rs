use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::PermissionLevel;
use crate::proto::types::command::CommandPermissionLevel;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
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

/// Sent by the server to update game-play related features, in particular permissions to access
/// these features for the client. It includes allowing the player to fly, build and mine, and
/// attack entities. Most of these flags should be checked server-side instead of using this packet
/// only. The client may also send this packet to the server when it updates one of these settings
/// through the in-game settings interface. The server should verify if the player actually has
/// permission to update those settings.
#[derive(Debug, Clone)]
pub struct AdventureSettings {
    /// A set of flags that specify certain properties of the player, such as whether or not it can
    /// fly and/or move through blocks.
    pub flags: u32,
    /// A permission level that specifies the kind of commands that the player is allowed to use.
    pub command_permission_level: CommandPermissionLevel,
    /// Much like flags, a set of flags that specify actions that the player is allowed to take,
    /// such as whether it is allowed to edit blocks, open doors etc.
    pub action_permissions: u32,
    /// The permission level of the player as it shows up in the player list built up using the
    /// PlayerList packet.
    pub permission_level: PermissionLevel,
    /// The use of this field is currently unknown.
    pub custom_stored_permissions: u32,
    /// A unique identifier of the player. This must be filled out with the entity unique ID of the
    /// player.
    pub player_unique_id: i64,
}

impl PacketType for AdventureSettings {
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
