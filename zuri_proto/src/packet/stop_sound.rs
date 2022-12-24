use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct StopSound {
    pub sound_name: String,
    pub stop_all: bool,
}

impl PacketType for StopSound {
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
