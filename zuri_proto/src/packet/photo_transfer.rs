#[derive(Debug)]
pub struct PhotoTransfer {
    pub photo_name: String,
    pub photo_data: Bytes,
    pub book_id: String,
    pub photo_type: u8,
    pub source_type: u8,
    pub owner_entity_unique_id: i64,
    pub new_photo_name: String,
}

impl Packet for PhotoTransfer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.photo_name.as_str());
        writer.byte_slice(&self.photo_data);
        writer.string(self.book_id.as_str());
        writer.u8(self.photo_type);
        writer.u8(self.source_type);
        writer.i64(self.owner_entity_unique_id);
        writer.string(self.new_photo_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            photo_name: reader.string(),
            photo_data: reader.byte_slice(),
            book_id: reader.string(),
            photo_type: reader.u8(),
            source_type: reader.u8(),
            owner_entity_unique_id: reader.i64(),
            new_photo_name: reader.string(),
        }
    }
}
