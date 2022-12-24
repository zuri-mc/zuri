use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct ServerStats {
    pub server_time: f32,
    pub network_time: f32,
}

impl PacketType for ServerStats {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.server_time);
        writer.f32(self.network_time);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            server_time: reader.f32(),
            network_time: reader.f32(),
        }
    }
}
