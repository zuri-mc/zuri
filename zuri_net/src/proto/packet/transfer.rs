use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct Transfer {
    pub address: String,
    pub port: u16,
}

impl PacketType for Transfer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.address.as_str());
        writer.u16(self.port);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            address: reader.string(),
            port: reader.u16(),
        }
    }
}
