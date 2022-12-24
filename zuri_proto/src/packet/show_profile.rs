use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct ShowProfile {
    pub xuid: String,
}

impl PacketType for ShowProfile {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.xuid.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            xuid: reader.string(),
        }
    }
}
