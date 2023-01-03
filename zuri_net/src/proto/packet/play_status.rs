use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PlayStatusType {
    LoginSuccess,
    LoginFailedClient,
    LoginFailedServer,
    PlayerSpawn,
    LoginFailedInvalidTenant,
    LoginFailedVanillaEdu,
    LoginFailedEduVanilla,
    LoginFailedServerFull,
    LoginFailedEditorVanilla,
    LoginFailedVanillaEditor,
}

/// Sent by the server to update a player on the play status. This includes failed statuses due to a
/// mismatched version, but also success statuses.
#[derive(Debug, Clone)]
pub struct PlayStatus {
    /// The status of the packet.
    pub status: PlayStatusType,
}

impl PacketType for PlayStatus {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.status.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { status: PlayStatusType::from_i32(reader.i32_be()).unwrap() }
    }
}
