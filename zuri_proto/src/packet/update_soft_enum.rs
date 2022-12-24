use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct UpdateSoftEnum {
    pub enum_type: String,
    pub options: Vec<String>,
    pub action_type: u8,
}

impl PacketType for UpdateSoftEnum {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.enum_type.as_str());
        writer.var_u32(self.options.len() as u32);
        self.options.iter().for_each(|option| writer.string(option.as_str()));
        writer.u8(self.action_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            enum_type: reader.string(),
            options: (0..reader.var_u32()).map(|_| reader.string()).collect(),
            action_type: reader.u8(),
        }
    }
}
