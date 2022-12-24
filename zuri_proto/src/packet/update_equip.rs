use bytes::Bytes;
use crate::io::{Reader, Writer};
use crate::packet::Packet;
use crate::types::inventory::Window;

#[derive(Debug)]
pub struct UpdateEquip {
    pub window: Window,
    pub window_type: u8,
    pub size: i32,
    pub entity_unique_id: i64,
    pub serialised_inventory_data: Bytes,
}

impl Packet for UpdateEquip {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.u8(self.window_type);
        writer.var_i32(self.size);
        writer.var_i64(self.entity_unique_id);
        writer.bytes(&self.serialised_inventory_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            window_type: reader.u8(),
            size: reader.var_i32(),
            entity_unique_id: reader.var_i64(),
            serialised_inventory_data: reader.bytes(),
        }
    }
}
