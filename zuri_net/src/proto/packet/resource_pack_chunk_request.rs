use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to request a chunk of data from a particular resource pack, that it has
/// obtained information about in a ResourcePackDataInfo packet.
#[derive(Debug, Clone)]
pub struct ResourcePackChunkRequest {
    /// The unique ID of the resource pack that the chunk of data is requested from.
    pub uuid: String,
    /// The requested chunk index of the chunk. It is a number that starts at zero and is
    /// incremented for each resource pack data chunk requested.
    pub chunk_index: u32,
}

impl PacketType for ResourcePackChunkRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.chunk_index);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            chunk_index: reader.u32(),
        }
    }
}
