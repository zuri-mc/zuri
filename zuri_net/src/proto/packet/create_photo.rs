use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct CreatePhoto {
    pub entity_unique_id: i64,
    pub photo_name: String,
    pub item_name: String,
}

impl PacketType for CreatePhoto {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.string(self.photo_name.as_str());
        writer.string(self.item_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            photo_name: reader.string(),
            item_name: reader.string(),
        }
    }
}
