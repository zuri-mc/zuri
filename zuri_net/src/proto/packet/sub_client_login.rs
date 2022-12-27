use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct SubClientLogin {
    pub connection_request: Bytes,
}

impl PacketType for SubClientLogin {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.connection_request);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            connection_request: reader.byte_slice(),
        }
    }
}
