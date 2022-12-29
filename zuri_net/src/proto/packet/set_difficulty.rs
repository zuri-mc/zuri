use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::Difficulty;

#[derive(Debug, Clone)]
pub struct SetDifficulty {
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
