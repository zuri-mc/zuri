use uuid::Uuid;
use zuri_net_derive::proto;
use crate::proto::ints::VarU32;

use crate::proto::types::container::ContainerType;
use crate::proto::types::inventory::Window;
use crate::proto::types::item::ItemInstance;

/// Sent by the client when it crafts a particular item. Note that this packet may be fully ignored,
/// as the transaction systems provide all the information necessary.
#[proto]
#[derive(Debug, Clone)]
pub struct CraftingEvent {
    /// The window that the player crafted in.
    pub window: Window,
    /// The container type of the window the player crafted in.
    pub container_type: ContainerType,
    /// The UUID of the recipe that was crafted. It is the UUID of the recipe that was sent in the
    /// CraftingData packet.
    pub recipe_uuid: Uuid,
    /// List of items that the player put into the recipe so that it could create the output items.
    /// These items are consumed in the process.
    #[len_type(VarU32)]
    pub input: Vec<ItemInstance>,
    /// List of items that were obtained as a result of crafting the recipe.
    #[len_type(VarU32)]
    pub output: Vec<ItemInstance>,
}
