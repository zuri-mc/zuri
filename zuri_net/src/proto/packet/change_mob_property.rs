use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct ChangeMobProperty {
    pub entity_unique_id: u64,
    pub property: String,
    pub bool_value: bool,
    pub string_value: String,
    pub int_value: i32,
    pub float_value: f32,
}

impl PacketType for ChangeMobProperty {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_unique_id);
        writer.string(self.property.as_str());
        writer.bool(self.bool_value);
        writer.string(self.string_value.as_str());
        writer.var_i32(self.int_value);
        writer.f32(self.float_value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.u64(),
            property: reader.string(),
            bool_value: reader.bool(),
            string_value: reader.string(),
            int_value: reader.var_i32(),
            float_value: reader.f32(),
        }
    }
}
