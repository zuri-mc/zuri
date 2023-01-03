use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to the server at the start of the game. It is sent to let the server know if
/// it supports the client-side blob cache. Clients such as Nintendo Switch do not support the
/// cache, and attempting to use it anyway will fail.
#[derive(Debug, Clone)]
pub struct ClientCacheStatus {
    /// Specifies if the blob cache is enabled. If false, the server should not attempt to use the
    /// blob cache. If true, it may do so, but it may also choose not to use it.
    pub enabled: bool,
}

impl PacketType for ClientCacheStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.enabled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { enabled: reader.bool() }
    }
}
