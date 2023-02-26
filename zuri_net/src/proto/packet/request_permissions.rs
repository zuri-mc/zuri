use zuri_net_derive::proto;

use crate::proto::types::world::PermissionLevel;

/// Sent from the client to the server to request permissions that the client does not currently
/// have. It can only be sent by operators and host in vanilla Minecraft.
#[proto]
#[derive(Debug, Clone)]
pub struct RequestPermissions {
    /// The unique ID of the player. The unique ID is unique for the entire world and is often used
    /// in packets. Most servers send an unique ID equal to the runtime ID.
    pub entity_unique_id: i64,
    /// The current permission level of the player.
    #[enum_header(u8)]
    pub permission_level: PermissionLevel,
    /// The requested permission flags.
    pub requested_permissions: u16,
}
