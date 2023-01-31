use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::PermissionLevel;

/// Sent from the client to the server to request permissions that the client does not currently
/// have. It can only be sent by operators and host in vanilla Minecraft.
#[derive(Debug, Clone)]
pub struct RequestPermissions {
    /// The unique ID of the player. The unique ID is unique for the entire world and is often used
    /// in packets. Most servers send an unique ID equal to the runtime ID.
    pub entity_unique_id: i64,
    /// The current permission level of the player.
    pub permission_level: PermissionLevel,
    /// The requested permission flags.
    pub requested_permissions: u16,
}

impl PacketType for RequestPermissions {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(self.permission_level.to_u8().unwrap());
        writer.u16(self.requested_permissions);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            permission_level: PermissionLevel::from_u8(reader.u8()).unwrap(),
            requested_permissions: reader.u16(),
        }
    }
}
