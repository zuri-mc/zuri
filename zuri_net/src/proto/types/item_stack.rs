use std::fmt::Debug;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use zuri_nbt::{encoding::NetworkLittleEndian, NBTTag};
use zuri_net_derive::proto;

use crate::encodable_enum;
use crate::proto::ints::{VarI32, VarU32};
use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::types::item::ItemStack;
use crate::proto::types::item_descriptor::ItemDescriptorCount;

encodable_enum!(
    #[derive(Debug, Clone)]
    pub enum StackRequestAction {
        TakeStackRequestAction = 0,
        PlaceStackRequestAction = 1,
        SwapStackRequestAction = 2,
        DropStackRequestAction = 3,
        DestroyStackRequestAction = 4,
        ConsumeStackRequestAction = 5,
        CreateStackRequestAction = 6,
        PlaceInContainerStackRequestAction = 7,
        TakeOutContainerStackRequestAction = 8,
        LabTableCombineStackRequestAction = 9,
        BeaconPaymentStackRequestAction = 10,
        MineBlockStackRequestAction = 11,
        CraftRecipeStackRequestAction = 12,
        AutoCraftRecipeStackRequestAction = 13,
        CraftCreativeStackRequestAction = 14,
        CraftRecipeOptionalStackRequestAction = 15,
        CraftGrindstoneRecipeStackRequestAction = 16,
        CraftLoomRecipeStackRequestAction = 17,
        CraftNonImplementedStackRequestAction = 18,
        CraftResultsDeprecatedStackRequestAction = 19,
    }
);

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum StackRequestActionType {
    Take,
    Place,
    Swap,
    Drop,
    Destroy,
    Consume,
    Create,
    PlaceInContainer,
    TakeOutContainer,
    LabTableCombine,
    BeaconPayment,
    MineBlock,
    CraftRecipe,
    CraftRecipeAuto,
    CraftCreative,
    CraftRecipeOptional,
    CraftGrindstone,
    CraftLoom,
    CraftNonImplementedDeprecated,
    CraftResultsDeprecated,
}

impl Default for StackRequestAction {
    fn default() -> Self {
        Self::CraftNonImplementedStackRequestAction(CraftNonImplementedStackRequestAction {})
    }
}

#[proto(i32)]
#[derive(Debug, Copy, Clone, Default)]
pub enum FilterCause {
    #[default]
    ServerChatPublic,
    ServerChatWhisper,
    SignText,
    AnvilText,
    BookAndQuillText,
    CommandBlockText,
    BlockActorDataText,
    JoinEventText,
    LeaveEventText,
    SlashCommandChat,
    CartographyText,
    KickCommand,
    TitleCommand,
    SummonCommand,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ItemStackResponseStatus {
    Ok,
    Error,
}

#[derive(Debug, Clone)]
pub struct ItemComponentEntry {
    pub name: String,
    pub data: NBTTag,
}

impl ItemComponentEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.nbt(&self.data, NetworkLittleEndian);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            data: reader.nbt(NetworkLittleEndian),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemEnchantments {
    pub slot: i32,
    pub enchantments: [Vec<EnchantmentInstance>; 3],
}

impl Writable for ItemEnchantments {
    fn write(&self, writer: &mut Writer) {
        writer.i32(self.slot);
        self.enchantments.iter().for_each(|enchantment| {
            writer.var_u32(enchantment.len() as u32);
            enchantment
                .iter()
                .for_each(|enchantment| enchantment.write(writer));
        });
    }
}

impl Readable<ItemEnchantments> for ItemEnchantments {
    fn read(reader: &mut Reader) -> Self {
        Self {
            slot: reader.i32(),
            enchantments: [
                (0..reader.var_u32())
                    .map(|_| EnchantmentInstance::read(reader))
                    .collect(),
                (0..reader.var_u32())
                    .map(|_| EnchantmentInstance::read(reader))
                    .collect(),
                (0..reader.var_u32())
                    .map(|_| EnchantmentInstance::read(reader))
                    .collect(),
            ],
        }
    }
}

#[proto]
#[derive(Debug, Clone)]
pub struct EnchantmentInstance {
    pub enchantment_type: u8,
    pub level: u8,
}

#[proto]
#[derive(Debug, Clone)]
pub struct EnchantmentOption {
    pub cost: VarU32,
    pub enchantments: ItemEnchantments,
    pub name: String,
    pub recipe_network_id: VarU32,
}

#[derive(Debug, Clone)]
pub struct ItemEntry {
    pub name: String,
    pub runtime_id: i16,
    pub component_based: bool,
}

impl ItemEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.i16(self.runtime_id);
        writer.bool(self.component_based);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            runtime_id: reader.i16(),
            component_based: reader.bool(),
        }
    }
}

#[proto]
#[derive(Debug, Clone, Default)]
pub struct ItemStackRequestEntry {
    pub request_id: VarI32,
    #[len_type(VarU32)]
    pub actions: Vec<StackRequestAction>,
    #[len_type(VarU32)]
    pub filter_strings: Vec<String>,
    pub filter_cause: FilterCause,
}

#[derive(Debug, Clone)]
pub struct ItemStackResponseEntry {
    pub status: ItemStackResponseStatus,
    pub request_id: i32,
    pub container_info: Vec<StackResponseContainerInfo>,
}

impl ItemStackResponseEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.status.to_u8().unwrap());
        writer.var_i32(self.request_id);
        if self.status == ItemStackResponseStatus::Ok {
            writer.var_u32(self.container_info.len() as u32);
            self.container_info
                .iter()
                .for_each(|container_info| container_info.write(writer));
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let status = ItemStackResponseStatus::from_u8(reader.u8()).unwrap();
        Self {
            status: status.clone(),
            request_id: reader.var_i32(),
            container_info: if status == ItemStackResponseStatus::Ok {
                (0..reader.var_u32())
                    .map(|_| StackResponseContainerInfo::read(reader))
                    .collect()
            } else {
                Vec::new()
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackRequestSlotInfo {
    pub container_id: u8,
    pub slot: u8,
    pub stack_network_id: i32,
}

impl StackRequestSlotInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.container_id);
        writer.u8(self.slot);
        writer.var_i32(self.stack_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            container_id: reader.u8(),
            slot: reader.u8(),
            stack_network_id: reader.var_i32(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DestroyStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
}

impl DestroyStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct DropStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub randomly: bool,
}

impl DropStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            randomly: reader.bool(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        writer.bool(self.randomly);
    }
}

#[derive(Debug, Clone)]
pub struct AutoCraftRecipeStackRequestAction {
    pub recipe_network_id: u32,
    pub times_crafted: u8,
    pub ingredients: Vec<ItemDescriptorCount>,
}

impl AutoCraftRecipeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_network_id: reader.u32(),
            times_crafted: reader.u8(),
            ingredients: (0..reader.var_u32())
                .map(|_| ItemDescriptorCount::read(reader))
                .collect(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u32(self.recipe_network_id);
        writer.u8(self.times_crafted);
        writer.var_u32(self.ingredients.len() as u32);
        self.ingredients
            .iter()
            .for_each(|ingredient| ingredient.write(writer));
    }
}

#[derive(Debug, Clone)]
pub struct BeaconPaymentStackRequestAction {
    pub primary_effect: i32,
    pub secondary_effect: i32,
}

impl BeaconPaymentStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            primary_effect: reader.var_i32(),
            secondary_effect: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.primary_effect);
        writer.var_i32(self.secondary_effect);
    }
}

#[derive(Debug, Clone)]
pub struct MineBlockStackRequestAction {
    pub hotbar_slot: i32,
    pub predicted_durability: i32,
    pub stack_network_id: i32,
}

impl MineBlockStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            hotbar_slot: reader.var_i32(),
            predicted_durability: reader.var_i32(),
            stack_network_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.hotbar_slot);
        writer.var_i32(self.predicted_durability);
        writer.var_i32(self.stack_network_id);
    }
}

#[derive(Debug, Clone)]
pub struct LabTableCombineStackRequestAction {}

impl LabTableCombineStackRequestAction {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }

    fn write(&self, _: &mut Writer) {}
}

#[derive(Debug, Clone)]
pub struct TakeStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl TakeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct SwapStackRequestAction {
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl SwapStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        self.source.write(writer);
        self.destination.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct TakeOutContainerStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl TakeOutContainerStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct ConsumeStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
}

impl ConsumeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct CraftCreativeStackRequestAction {
    pub creative_item_network_id: u32,
}

impl CraftCreativeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            creative_item_network_id: reader.u32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.creative_item_network_id);
    }
}

#[derive(Debug, Clone)]
pub struct CraftGrindstoneRecipeStackRequestAction {
    pub recipe_network_id: u32,
    pub cost: i32,
}

impl CraftGrindstoneRecipeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_network_id: reader.u32(),
            cost: reader.i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipe_network_id);
        writer.var_i32(self.cost);
    }
}

#[derive(Debug, Clone)]
pub struct CraftLoomRecipeStackRequestAction {
    pub pattern: String,
}

impl CraftLoomRecipeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            pattern: reader.string(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.pattern.as_str());
    }
}

#[derive(Debug, Clone)]
pub struct CraftNonImplementedStackRequestAction {}

impl CraftNonImplementedStackRequestAction {
    pub fn read(_reader: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug, Clone)]
pub struct CraftRecipeOptionalStackRequestAction {
    pub recipe_network_id: u32,
    pub filter_string_index: i32,
}

impl CraftRecipeOptionalStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_network_id: reader.u32(),
            filter_string_index: reader.i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipe_network_id);
        writer.i32(self.filter_string_index);
    }
}

#[derive(Debug, Clone)]
pub struct CraftRecipeStackRequestAction {
    pub recipe_network_id: u32,
}

impl CraftRecipeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_network_id: reader.u32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipe_network_id);
    }
}

#[derive(Debug, Clone)]
pub struct CraftResultsDeprecatedStackRequestAction {
    pub result_items: Vec<ItemStack>,
    pub times_crafted: u8,
}

impl CraftResultsDeprecatedStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            result_items: (0..reader.var_u32())
                .map(|_| ItemStack::read(reader))
                .collect(),
            times_crafted: reader.u8(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.result_items.len() as u32);
        self.result_items.iter().for_each(|item| item.write(writer));
        writer.u8(self.times_crafted);
    }
}

#[derive(Debug, Clone)]
pub struct CreateStackRequestAction {
    pub results_slot: u8,
}

impl CreateStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            results_slot: reader.u8(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.results_slot);
    }
}

#[derive(Debug, Clone)]
pub struct PlaceInContainerStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl PlaceInContainerStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct PlaceStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl PlaceStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }
}

#[derive(Debug, Clone)]
pub struct StackResponseContainerInfo {
    pub container_id: u8,
    pub slot_info: Vec<StackResponseSlotInfo>,
}

impl StackResponseContainerInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.container_id);
        writer.var_u32(self.slot_info.len() as u32);
        self.slot_info
            .iter()
            .for_each(|slot_info| slot_info.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            container_id: reader.u8(),
            slot_info: (0..reader.var_u32())
                .map(|_| StackResponseSlotInfo::read(reader))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackResponseSlotInfo {
    pub slot: u8,
    pub hotbar_slot: u8,
    pub count: u8,
    pub stack_network_id: i32,
    pub custom_name: String,
    pub durability_correction: i32,
}

impl StackResponseSlotInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.slot);
        writer.u8(self.hotbar_slot);
        writer.u8(self.count);
        writer.var_i32(self.stack_network_id);
        writer.string(self.custom_name.as_str());
        writer.var_i32(self.durability_correction);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            slot: reader.u8(),
            hotbar_slot: reader.u8(),
            count: reader.u8(),
            stack_network_id: reader.var_i32(),
            custom_name: reader.string(),
            durability_correction: reader.var_i32(),
        }
    }
}
