use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ShowProfile {
    pub xuid: String,
}

impl Packet for ShowProfile {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.xuid.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            xuid: reader.string(),
        }
    }
}
