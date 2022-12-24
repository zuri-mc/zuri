use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server to update a player on the play status. This includes failed statuses due to a mismatched version,
/// but also success statuses.
#[derive(Debug)]
pub struct PlayStatus {
    /// The status of the packet.
    pub status: PlayStatusType,
}

impl Packet for PlayStatus {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(num::ToPrimitive::to_i32(&self.status).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { status: num::FromPrimitive::from_i32(reader.i32_be()).unwrap() }
    }
}
