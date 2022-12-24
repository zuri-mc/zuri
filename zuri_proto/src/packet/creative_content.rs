use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct CreativeContent {
    pub items: Vec<CreativeItem>,
}

impl Packet for CreativeContent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| CreativeItem::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct CreativeItem {
    pub creative_item_network_id: u32,
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
