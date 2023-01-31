use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::item::UseItemMethod;

/// Sent by the server to notify client that it should be done using the item it is currently using.
#[derive(Debug, Clone)]
pub struct CompletedUsingItem {
    /// The item ID of the item that the client completed using. This should typically be the ID of
    /// the item held in the hand.
    pub used_item_id: i16,
    /// The method of the using of the item that was completed.
    pub use_method: UseItemMethod,
}

impl PacketType for CompletedUsingItem {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.used_item_id);
        writer.i32(self.use_method.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            used_item_id: reader.i16(),
            use_method: UseItemMethod::from_i32(reader.i32()).unwrap(),
        }
    }
}
