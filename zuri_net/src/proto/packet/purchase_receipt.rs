use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct PurchaseReceipt {
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
