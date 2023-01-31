use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::Difficulty;

/// Sent by the server to update the client-side difficulty of the client. The actual effect of this
/// packet on the client isn't very significant, as the difficulty is handled server-side.
#[derive(Debug, Clone)]
pub struct SetDifficulty {
    /// The new difficulty that the world has.
    pub difficulty: Difficulty,
}

impl PacketType for SetDifficulty {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.difficulty.to_u32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { difficulty: Difficulty::from_u32(reader.var_u32()).unwrap() }
    }
}
