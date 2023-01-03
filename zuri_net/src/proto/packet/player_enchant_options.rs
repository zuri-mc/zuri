use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item_stack::EnchantmentOption;

/// Sent by the server to update the enchantment options displayed when the user opens the
/// enchantment table and puts an item in. This packet was added in 1.16 and allows the server to
/// decide on the enchantments that can be selected by the player. The PlayerEnchantOptions packet
/// should be sent once for every slot update of the enchantment table. The vanilla server sends an
/// empty PlayerEnchantOptions packet when the player opens the enchantment table (air is present in
/// the enchantment table slot) and sends the packet with actual enchantments in it when items are
/// put in that can have enchantments.
#[derive(Debug, Clone)]
pub struct PlayerEnchantOptions {
    /// A list of possible enchantment options for the item that was put into the enchantment table.
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
