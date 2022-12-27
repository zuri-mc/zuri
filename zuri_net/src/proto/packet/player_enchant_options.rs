use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item_stack::EnchantmentOption;

#[derive(Debug)]
pub struct PlayerEnchantOptions {
    pub options: Vec<EnchantmentOption>,
}

impl PacketType for PlayerEnchantOptions {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.options.len() as u32);
        self.options.iter().for_each(|option| option.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { options: (0..reader.var_u32()).map(|_| EnchantmentOption::read(reader)).collect() }
    }
}
