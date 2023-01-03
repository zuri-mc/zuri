use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to request photo information from the server.
#[derive(Debug, Clone)]
pub struct PhotoInfoRequest {
    /// The ID of the photo.
    pub photo_id: i64,
}

impl PacketType for PhotoInfoRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.photo_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { photo_id: reader.var_i64() }
    }
}
