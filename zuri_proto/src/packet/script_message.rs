use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ScriptMessage {
    pub identifier: String,
    pub data: Bytes,
}

impl Packet for ScriptMessage {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.identifier.as_str());
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            identifier: reader.string(),
            data: reader.byte_slice(),
        }
    }
}
