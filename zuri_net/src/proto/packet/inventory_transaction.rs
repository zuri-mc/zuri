use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::inventory::*;

/// Sent by the client. It essentially exists out of multiple sub-packets, each of which have
/// something to do with the inventory in one way or another. Some of these sub-packets directly
/// relate to the inventory, others relate to interaction with the world, that could potentially
/// result in a change in the inventory.
#[derive(Debug, Clone)]
pub struct InventoryTransaction {
    /// ID that is only non-zero at times when sent by the client. The server should always send
    /// zero for this. When this field is not zero, the legacy set item slots list below will have
    /// values in it. legacy request ID ties in with the ItemStackResponse packet. If this field is
    /// non-zero, the server should respond with an ItemStackResponse packet. Some inventory actions
    /// such as dropping an item out of the hotbar are still one using this packet, and the
    /// ItemStackResponse packet needs to tie in with it.
    pub legacy_request_id: i32,
    /// Only present if the legacy request ID is non-zero. These item slots inform the server of the
    /// slots that were changed during the inventory transaction, and the server should send back an
    /// ItemStackResponse packet with these slots present in it. (Or false with no slots, if
    /// rejected.)
    pub legacy_set_item_slots: Vec<LegacySetItemSlot>,
    /// List of actions that took place, that form the inventory transaction together. Each of these
    /// actions hold one slot in which one item was changed to another. In general, the combination
    /// of all of these actions results in a balanced inventory transaction. This should be checked
    /// to ensure that no items are cheated into the inventory.
    pub actions: Vec<InventoryAction>,
    /// Data object that holds data specific to the type of transaction that the TransactionPacket
    /// held. Its concrete type must be one of Normal, Mismatch, UseItem, UseItemOnEntity or
    /// ReleaseItem. If empty, the transaction will be assumed to of type Normal.
    pub transaction_data: InventoryTransactionData,
}

impl PacketType for InventoryTransaction {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.legacy_request_id);
        if self.legacy_request_id != 0 {
            writer.var_u32(self.legacy_set_item_slots.len() as u32);
            self.legacy_set_item_slots
                .iter()
                .for_each(|slot| slot.write(writer));
        }

        // todo: split the type from the data when writing
        //writer.var_u32(self.transaction_data.transaction_type().to_u32().unwrap());

        writer.var_u32(self.actions.len() as u32);
        self.actions.iter().for_each(|action| action.write(writer));

        self.transaction_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        let legacy_request_id = reader.var_i32();
        let legacy_set_item_slots = if legacy_request_id != 0 {
            (0..reader.var_u32())
                .map(|_| LegacySetItemSlot::read(reader))
                .collect()
        } else {
            Vec::new()
        };
        // todo: split the type from the data when writing
        //let transaction_type = InventoryTransactionType::from_u32(reader.var_u32()).unwrap();
        Self {
            legacy_request_id,
            legacy_set_item_slots,
            actions: (0..reader.var_u32())
                .map(|_| InventoryAction::read(reader))
                .collect(),
            transaction_data: InventoryTransactionData::read(reader),
        }
    }
}
