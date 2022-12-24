use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct RequestPermissions {
    pub entity_unique_id: i64,
    pub permission_level: PermissionLevel,
    pub requested_permissions: u16,
}

impl Packet for RequestPermissions {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(num::ToPrimitive::to_u8(&self.permission_level).unwrap());
        writer.u16(self.requested_permissions);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            permission_level: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            requested_permissions: reader.u16(),
        }
    }
}
