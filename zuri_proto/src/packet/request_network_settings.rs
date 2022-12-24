use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct RequestNetworkSettings {
    pub client_protocol: i32,
}

impl Packet for RequestNetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.client_protocol);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_protocol: reader.i32_be(),
        }
    }
}
