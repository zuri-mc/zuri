/// Sent by the client to the server and the server to the client to make the other side aware of the new item that an
/// entity is holding. It is used to show the item in the hand of entities such as zombies too.
#[derive(Debug)]
pub struct MobEquipment {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The new item held after sending the MobEquipment packet. The entity will be shown holding that item to the
    /// player it was sent to.
    pub new_item: ItemInstance,
    /// The slot in the inventory that was held. This is the same as hotbar_slot, and only remains for backwards
    /// compatibility.
    pub inventory_slot: u8,
    /// The slot in the hot bar that was held. It is the same as InventorySlot, which is only here for backwards
    /// compatibility purposes.
    pub hotbar_slot: u8,
    /// The window that had its equipped item changed. This is usually the window ID of the normal inventory, but may
    /// also be something else, for example with the off hand.
    pub window: Window,
}

impl Packet for MobEquipment {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        self.new_item.write(writer);

        writer.u8(self.inventory_slot);
        writer.u8(self.hotbar_slot);
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),

            new_item: ItemInstance::read(reader),

            inventory_slot: reader.u8(),
            hotbar_slot: reader.u8(),
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap();
        }
    }
}
