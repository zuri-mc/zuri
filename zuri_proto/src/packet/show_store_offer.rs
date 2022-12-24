use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct ShowStoreOffer {
    pub offer_id: String,
    pub show_all: bool,
}

impl PacketType for ShowStoreOffer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.offer_id.as_str());
        writer.bool(self.show_all);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            offer_id: reader.string(),
            show_all: reader.bool(),
        }
    }
}
