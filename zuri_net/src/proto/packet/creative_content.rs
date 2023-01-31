use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item::ItemStack;

/// Sent by the server to set the creative inventory's content for a player. Introduced in 1.16,
/// this packet replaces the previous method - sending an InventoryContent packet with creative
/// inventory window ID. As of v1.16.100, this packet must be sent during the login sequence. Not
/// sending it will stop the client from joining the server.
#[derive(Debug, Clone)]
pub struct CreativeContent {
    /// A list of the items that should be added to the creative inventory.
    pub items: Vec<CreativeItem>,
}

impl PacketType for CreativeContent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|i| i.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| CreativeItem::read(reader)).collect() }
    }
}

/// A creative item present in the creative inventory.
#[derive(Debug, Clone)]
pub struct CreativeItem {
    /// A unique ID for the creative item. It has to be unique for each creative item sent to the
    /// client. An incrementing ID per creative item does the job.
    pub creative_item_network_id: u32,
    /// The item that should be added to the creative inventory.
    pub item: ItemStack,
}

impl CreativeItem {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.creative_item_network_id);
        self.item.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            creative_item_network_id: reader.var_u32(),
            item: ItemStack::read(reader),
        }
    }
}
