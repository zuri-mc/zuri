use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item_stack::ItemStackRequestEntry;

/// Sent by the client to change item stacks in an inventory. It is essentially a replacement of the
/// InventoryTransaction packet added in 1.16 for inventory specific actions, such as moving items
/// around or crafting. The InventoryTransaction packet is still used for actions such as placing
/// blocks and interacting with entities.
#[derive(Debug, Clone)]
pub struct ItemStackRequest {
    /// A list of item stack requests. These requests are all separate, but the client buffers the
    /// requests, so you might find multiple unrelated requests in this packet.
    pub requests: Vec<ItemStackRequestEntry>,
}

impl PacketType for ItemStackRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.requests.len() as u32);
        self.requests.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { requests: (0..reader.var_u32()).map(|_| ItemStackRequestEntry::read(reader)).collect() }
    }
}
