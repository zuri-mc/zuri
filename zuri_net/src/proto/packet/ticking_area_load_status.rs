use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct TickingAreasLoadStatus {
    pub preload: bool,
}

impl PacketType for TickingAreasLoadStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.preload);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            preload: reader.bool(),
        }
    }
}
