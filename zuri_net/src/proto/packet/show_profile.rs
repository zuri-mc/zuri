use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
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
