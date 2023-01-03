use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Allows players to export photos from their portfolios into items in their inventory. This packet
/// only works on the Education Edition version of Minecraft.
#[derive(Debug, Clone)]
pub struct CreatePhoto {
    /// The unique ID of the entity.
    pub entity_unique_id: i64,
    /// The name of the photo.
    pub photo_name: String,
    /// The name of the photo as an item.
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
