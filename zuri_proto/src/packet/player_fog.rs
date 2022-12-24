use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct PlayerFog {
    pub stack: Vec<String>,
}

impl PacketType for PlayerFog {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.stack.len() as u32);
        self.stack.iter().for_each(|stack| writer.string(stack.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { stack: (0..reader.var_u32()).map(|_| reader.string()).collect() }
    }
}
