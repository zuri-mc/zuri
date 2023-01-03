use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to stop a sound playing to the player, such as a playing music disk track or
/// other long-lasting sounds.
#[derive(Debug, Clone)]
pub struct StopSound {
    /// The name of the sound that should be stopped from playing. If no sound with this name is
    /// currently active, the packet is ignored.
    pub sound_name: String,
    /// Specifies if all sounds currently playing to the player should be stopped. If set to true,
    /// the `sound_name` field may be left empty.
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
