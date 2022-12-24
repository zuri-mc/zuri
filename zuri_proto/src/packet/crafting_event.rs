use uuid::Uuid;
use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::inventory::Window;
use crate::types::item::ItemInstance;
use crate::types::container::ContainerType;

/// Sent by the client when it crafts a particular item. Note that this packet may be fully ignored, as the transaction
/// systems provide all the information necessary.
#[derive(Debug)]
pub struct CraftingEvent {
    /// The window that the player crafted in.
    pub window: Window,
    /// The container type of the window the player crafted in.
    pub container_type: ContainerType,
    /// The UUID of the recipe that was crafted. It is the UUID of the recipe that was sent in the CraftingData packet.
    pub recipe_uuid: Uuid,
    /// List of items that the player put into the recipe so that it could create the output items. These items are
    /// consumed in the process.
    pub input: Vec<ItemInstance>,
    /// List of items that were obtained as a result of crafting the recipe.
    pub output: Vec<ItemInstance>,
}

impl Packet for CraftingEvent {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.container_type).unwrap());
        writer.uuid(self.recipe_uuid);

        writer.var_u32(self.input.len() as u32);
        self.input.iter().for_each(|item| item.write(writer));

        writer.var_u32(self.output.len() as u32);
        self.output.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            container_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            recipe_uuid: reader.uuid(),
            input: (0..reader.var_u32()).map(|_| ItemInstance::read(reader)).collect(),
            output: (0..reader.var_u32()).map(|_| ItemInstance::read(reader)).collect(),
        }
    }
}
