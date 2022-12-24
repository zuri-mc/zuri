use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct DeathInfo {
    pub cause: String,
    pub messages: Vec<String>,
}

impl PacketType for DeathInfo {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.cause.as_str());
        writer.var_u32(self.messages.len() as u32);
        self.messages.iter().for_each(|m| writer.string(m.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            cause: reader.string(),
            messages: (0..reader.var_u32()).map(|_| reader.string()).collect(),
        }
    }
}
