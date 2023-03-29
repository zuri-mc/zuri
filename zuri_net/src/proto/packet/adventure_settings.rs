use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

use crate::proto::types::command::CommandPermissionLevel;
use crate::proto::types::world::PermissionLevel;

// todo: flags
#[derive(Debug, Clone)]
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

// todo: flags
#[derive(Debug, Clone)]
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
#[proto]
#[derive(Debug, Clone)]
pub struct AdventureSettings {
    /// A set of flags that specify certain properties of the player, such as whether or not it can
    /// fly and/or move through blocks.
    pub flags: VarU32,
    /// A permission level that specifies the kind of commands that the player is allowed to use.
    pub command_permission_level: CommandPermissionLevel,
    /// Much like flags, a set of flags that specify actions that the player is allowed to take,
    /// such as whether it is allowed to edit blocks, open doors etc.
    pub action_permissions: VarU32,
    /// The permission level of the player as it shows up in the player list built up using the
    /// PlayerList packet.
    pub permission_level: PermissionLevel,
    /// The use of this field is currently unknown.
    pub custom_stored_permissions: VarU32,
    /// A unique identifier of the player. This must be filled out with the entity unique ID of the
    /// player.
    pub player_unique_id: i64,
}
