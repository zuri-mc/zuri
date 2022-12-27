use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ShowCreditsStatus {
    Start,
    End,
}

#[derive(Debug)]
pub struct ShowCredits {
    pub player_runtime_id: u64,
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
