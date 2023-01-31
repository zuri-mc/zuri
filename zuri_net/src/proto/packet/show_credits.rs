use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ShowCreditsStatus {
    Start,
    End,
}

/// Sent by the server to show the Minecraft credits screen to the client. It is typically sent when
/// the player beats the ender dragon and leaves the End.
#[derive(Debug, Clone)]
pub struct ShowCredits {
    /// The entity runtime ID of the player to show the credits to. It's not clear why this field is
    /// actually here in the first place.
    pub player_runtime_id: u64,
    /// The status type of the credits. It either starts or stops the credits.
    pub status_type: ShowCreditsStatus,
}

impl PacketType for ShowCredits {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.player_runtime_id);
        writer.var_i32(self.status_type.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_runtime_id: reader.var_u64(),
            status_type: ShowCreditsStatus::from_i32(reader.var_i32()).unwrap(),
        }
    }
}
