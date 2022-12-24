use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SetCommandsEnabled {
    pub enabled: bool,
}

impl Packet for SetCommandsEnabled {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.enabled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            enabled: reader.bool(),
        }
    }
}
