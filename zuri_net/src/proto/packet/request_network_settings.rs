use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct RequestNetworkSettings {
    pub client_protocol: i32,
}

impl PacketType for RequestNetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.client_protocol);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_protocol: reader.i32_be(),
        }
    }
}
