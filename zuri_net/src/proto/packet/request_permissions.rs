use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::PermissionLevel;

#[derive(Debug)]
pub struct RequestPermissions {
    pub entity_unique_id: i64,
    pub permission_level: PermissionLevel,
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
