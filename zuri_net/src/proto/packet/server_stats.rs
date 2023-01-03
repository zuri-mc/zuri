use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent from the server to update the client on server statistics. It is purely used for telemetry.
#[derive(Debug, Clone)]
pub struct ServerStats {
    /// The server tick when the statistics were collected.
    pub server_time: f32,
    /// The latency between the client and the server, as measured by the server.
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
