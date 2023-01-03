use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent from the server to the client to change one of the properties of a mob client-side.
#[derive(Debug, Clone)]
pub struct ChangeMobProperty {
    /// The unique ID of the entity whose property is being changed.
    pub entity_unique_id: u64,
    /// The name of the property being updated.
    pub property: String,
    /// Set if the property value is a bool. If the type is not a bool, this field is ignored.
    pub bool_value: bool,
    /// Set if the property value is a string. If the type is not a string, this field is ignored.
    pub string_value: String,
    /// Set if the property value is an int. If the type is not an int, this field is ignored.
    pub int_value: i32,
    /// Set if the property value is a float. If the type is not a float, this field is ignored.
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
