use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};
use crate::types::world::Difficulty;

#[derive(Debug)]
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
