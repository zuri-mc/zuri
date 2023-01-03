use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to the client to notify the client of a ticking area's loading status.
#[derive(Debug, Clone)]
pub struct TickingAreasLoadStatus {
    /// True if the server is waiting for the area's preload.
    pub preload: bool,
}

impl PacketType for TickingAreasLoadStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.preload);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { preload: reader.bool() }
    }
}
