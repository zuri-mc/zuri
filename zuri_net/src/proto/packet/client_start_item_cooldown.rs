use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct ClientStartItemCooldown {
    pub category: String,
    pub duration: i32,
}

impl PacketType for ClientStartItemCooldown {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.category.as_str());
        writer.var_i32(self.duration);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            category: reader.string(),
            duration: reader.var_i32(),
        }
    }
}
