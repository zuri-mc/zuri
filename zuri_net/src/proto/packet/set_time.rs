use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to update the current time client-side. The client actually advances time client-side by itself,
/// so this packet does not need to be sent each tick. It is a means of synchronising time between server and client.
#[derive(Debug)]
pub struct SetTime {
    /// The current time. The time is not limited to 24000 (time of day), but continues progressing after that.
    pub time: i32,
}

impl PacketType for SetTime {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.time);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            time: reader.var_i32(),
        }
    }
}
