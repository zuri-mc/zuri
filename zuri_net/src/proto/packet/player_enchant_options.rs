use crate::proto::ints::VarU32;
use crate::proto::types::item_stack::EnchantmentOption;
use zuri_net_derive::proto;

/// Sent by the server to update the enchantment options displayed when the user opens the
/// enchantment table and puts an item in. This packet was added in 1.16 and allows the server to
/// decide on the enchantments that can be selected by the player. The PlayerEnchantOptions packet
/// should be sent once for every slot update of the enchantment table. The vanilla server sends an
/// empty PlayerEnchantOptions packet when the player opens the enchantment table (air is present in
/// the enchantment table slot) and sends the packet with actual enchantments in it when items are
/// put in that can have enchantments.
#[proto]
#[derive(Debug, Clone)]
pub struct PlayerEnchantOptions {
    /// A list of possible enchantment options for the item that was put into the enchantment table.
    #[len_type(VarU32)]
    pub options: Vec<EnchantmentOption>,
}
