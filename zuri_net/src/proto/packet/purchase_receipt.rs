use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to the server to notify the server it purchased an item from the Marketplace
/// store that was offered by the server. The packet is only used for partnered servers.
#[derive(Debug, Clone)]
pub struct PurchaseReceipt {
    /// A list of receipts, or proofs of purchases, for the offers that have been purchased by the
    /// player. This is used for server-side verification of the purchase.
    pub receipts: Vec<String>,
}

impl PacketType for PurchaseReceipt {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.receipts.len() as u32);
        self.receipts.iter().for_each(|receipt| writer.string(receipt.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { receipts: (0..reader.var_u32()).map(|_| reader.string()).collect() }
    }
}
