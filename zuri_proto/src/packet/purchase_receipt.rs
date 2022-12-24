use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct PurchaseReceipt {
    pub receipts: Vec<String>,
}

impl Packet for PurchaseReceipt {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.receipts.len() as u32);
        self.receipts.iter().for_each(|receipt| writer.string(receipt.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { receipts: (0..reader.var_u32()).map(|_| reader.string()).collect() }
    }
}
