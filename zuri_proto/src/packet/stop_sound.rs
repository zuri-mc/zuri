use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct StopSound {
    pub sound_name: String,
    pub stop_all: bool,
}

impl Packet for StopSound {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.sound_name.as_str());
        writer.bool(self.stop_all);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_name: reader.string(),
            stop_all: reader.bool(),
        }
    }
}
