use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SetDifficulty {
    pub difficulty: Difficulty,
}

impl Packet for SetDifficulty {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(num::ToPrimitive::to_u32(&self.difficulty).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            difficulty: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
        }
    }
}
