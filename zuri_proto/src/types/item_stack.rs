use crate::io::{Reader, Writer};
encodable_enum!(
    #[derive(Debug)]
    pub enum StackRequestAction2 {
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

impl Default for StackRequestAction2 {
    fn default() -> Self {
        Self::CraftNonImplementedStackRequestAction(CraftNonImplementedStackRequestAction {})
    }
}

#[derive(Debug)]
pub struct ItemComponentEntry {
    pub name: String,
    //pub data: dyn Any, // TODO: NBT
}

impl ItemComponentEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        // TODO: NBT (data)
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            // data: {
            //     // TODO: NBT
            // },
        }
    }
}

pub trait ItemDescriptor: Debug {
    fn write(&self, writer: &mut Writer);
    fn descriptor_type(&self) -> ItemDescriptorType;
}

#[derive(Debug)]
pub struct ItemDescriptorCount {
    pub item_descriptor: Box<dyn ItemDescriptor>,
    pub count: i32,
}

impl ItemDescriptorCount {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&(self.item_descriptor.descriptor_type())).unwrap());
        self.item_descriptor.write(writer);
        writer.var_i32(self.count);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_descriptor: match num::FromPrimitive::from_u8(reader.u8()).unwrap() {
                ItemDescriptorType::Invalid => Box::from(InvalidItemDescriptor::read(reader)),
                ItemDescriptorType::Default => Box::from(DefaultItemDescriptor::read(reader)),
                ItemDescriptorType::MoLang => Box::from(MoLangItemDescriptor::read(reader)),
                ItemDescriptorType::ItemTag => Box::from(ItemTagItemDescriptor::read(reader)),
                ItemDescriptorType::Deferred => Box::from(DeferredItemDescriptor::read(reader)),
            },
            count: reader.var_i32(),
        }
    }
}

impl Default for ItemDescriptorCount {
    fn default() -> Self {
        Self {
            item_descriptor: Box::from(InvalidItemDescriptor {}),
            count: 0,
        }
    }
}

#[derive(Debug)]
pub struct ItemEnchantments {
    pub slot: i32,
    pub enchantments: [Vec<EnchantmentInstance>; 3],
}

impl ItemEnchantments {
    pub fn write(&self, writer: &mut Writer) {
        writer.i32(self.slot);
        self.enchantments.iter().for_each(|enchantment| {
            writer.var_u32(enchantment.len() as u32);
            enchantment.iter().for_each(|enchantment| enchantment.write(writer));
        });
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            slot: reader.i32(),
            enchantments: [
                (0..reader.var_u32()).map(|_| EnchantmentInstance::read(reader)).collect(),
                (0..reader.var_u32()).map(|_| EnchantmentInstance::read(reader)).collect(),
                (0..reader.var_u32()).map(|_| EnchantmentInstance::read(reader)).collect(),
            ],
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub struct ItemInstance {
    pub stack_network_id: i32,
    pub stack: ItemStack,
}

impl ItemInstance {
    pub fn write(&self, writer: &mut Writer) {
        writer.i32(self.stack_network_id);
        self.stack.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            stack_network_id: reader.i32(),
            stack: ItemStack::read(reader),
        }
    }
}

#[derive(Debug, Default)]
pub struct ItemStack {
    pub network_id: i32,
    pub metadata_value: u32,
    pub block_runtime_id: i32,
    pub count: u16,
    //pub nbt_data: dyn Any, // TODO: NBT
    pub can_be_placed_on: Vec<String>,
    pub can_break: Vec<String>,
    pub has_network_id: bool,
}

impl ItemStack {
    pub fn write(&self, writer: &mut Writer) {
        // writer.write_ItemType(self.item_type);
        // writer.i32(self.block_runtime_id);
        // writer.u16(self.count);
        // writer.write_TODO(self.LEN);
        // writer.write_String(self.nbt_data);
        // writer.write_TODO(self.LEN);
        // writer.write_String(self.can_be_placed_on);
        // writer.write_TODO(self.LEN);
        // writer.write_String(self.can_break);
        // writer.bool(self.has_network_id);
        todo!()
    }

    pub fn read(reader: &mut Reader) -> Self {
        todo!()
        // Self {
        //     item_type: reader.read_ItemType(),
        //     block_runtime_id: reader.i32(),
        //     count: reader.u16(),
        //     LEN: reader.read_TODO(),
        //     nbt_data: reader.read_String(),
        //     LEN: reader.read_TODO(),
        //     can_be_placed_on: reader.read_String(),
        //     LEN: reader.read_TODO(),
        //     can_break: reader.read_String(),
        //     has_network_id: reader.bool(),
        // }
    }
}

#[derive(Debug, Default)]
pub struct ItemStackRequestEntry {
    pub request_id: i32,
    pub actions: Vec<StackRequestAction2>,
    pub filter_strings: Vec<String>,
    pub filter_cause: i32,
}

impl ItemStackRequestEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.request_id);
        writer.var_u32(self.actions.len() as u32);
        self.actions.iter().for_each(|action| {
            action.write(writer);
        });
        writer.var_u32(self.filter_strings.len() as u32);
        self.filter_strings.iter().for_each(|filter_string| writer.string(filter_string.as_str()));
        writer.i32(self.filter_cause);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            request_id: reader.var_i32(),
            actions: (0..reader.var_u32()).map(|_| {
                StackRequestAction2::read(reader)
            }).collect(),
            filter_strings: (0..reader.var_u32()).map(|_| reader.string()).collect(),
            filter_cause: reader.i32(),
        }
    }
}

#[derive(Debug)]
pub struct ItemStackResponseEntry {
    pub status: ItemStackResponseStatus,
    pub request_id: i32,
    pub container_info: Vec<StackResponseContainerInfo>,
}

impl ItemStackResponseEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.status).unwrap());
        writer.var_i32(self.request_id);
        if self.status == ItemStackResponseStatus::Ok {
            writer.var_u32(self.container_info.len() as u32);
            self.container_info.iter().for_each(|container_info| container_info.write(writer));
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let status: ItemStackResponseStatus = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            status: status.clone(),
            request_id: reader.var_i32(),
            container_info: if status == ItemStackResponseStatus::Ok { (0..reader.var_u32()).map(|_| StackResponseContainerInfo::read(reader)).collect() } else { Vec::new() },
        }
    }
}

#[derive(Debug)]
pub struct ItemTagItemDescriptor {
    tag: String,
}

impl ItemTagItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            tag: reader.string(),
        }
    }
}

impl ItemDescriptor for ItemTagItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.tag.as_str());
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Deferred
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for MineBlockStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.hotbar_slot);
        writer.var_i32(self.predicted_durability);
        writer.var_i32(self.stack_network_id);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::MineBlock
    }
}

#[derive(Debug)]
pub struct LabTableCombineStackRequestAction {}

impl LabTableCombineStackRequestAction {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

impl StackRequestAction for LabTableCombineStackRequestAction {
    fn write(&self, _: &mut Writer) {}

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::LabTableCombine
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for TakeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Take
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for SwapStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Swap
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for TakeOutContainerStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::TakeOutContainer
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for ConsumeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Consume
    }
}

#[derive(Debug)]
pub struct CraftCreativeStackRequestAction {
    pub creative_item_network_id: u32,
}

impl CraftCreativeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            creative_item_network_id: reader.u32(),
        }
    }
}

impl StackRequestAction for CraftCreativeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.creative_item_network_id);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftCreative
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for CraftGrindstoneRecipeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipe_network_id);
        writer.var_i32(self.cost);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftGrindstone
    }
}

#[derive(Debug)]
pub struct CraftLoomRecipeStackRequestAction {
    pub pattern: String,
}

impl CraftLoomRecipeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            pattern: reader.string(),
        }
    }
}

impl StackRequestAction for CraftLoomRecipeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.pattern.as_str());
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftLoom
    }
}

#[derive(Debug)]
pub struct CraftNonImplementedStackRequestAction {}

impl CraftNonImplementedStackRequestAction {
    pub fn read(_reader: &mut Reader) -> Self {
        Self {}
    }
}

impl StackRequestAction for CraftNonImplementedStackRequestAction {
    fn write(&self, _: &mut Writer) {}

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Take
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for CraftRecipeOptionalStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipe_network_id);
        writer.i32(self.filter_string_index);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftRecipeOptional
    }
}

#[derive(Debug)]
pub struct CraftRecipeStackRequestAction {
    pub recipe_network_id: u32,
}

impl CraftRecipeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_network_id: reader.u32(),
        }
    }
}

impl StackRequestAction for CraftRecipeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipe_network_id);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftRecipe
    }
}

#[derive(Debug)]
pub struct CraftResultsDeprecatedStackRequestAction {
    pub result_items: Vec<ItemStack>,
    pub times_crafted: u8,
}

impl CraftResultsDeprecatedStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            result_items: (0..reader.var_u32()).map(|_| ItemStack::read(reader)).collect(),
            times_crafted: reader.u8(),
        }
    }
}

impl StackRequestAction for CraftResultsDeprecatedStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.result_items.len() as u32);
        self.result_items.iter().for_each(|item| item.write(writer));
        writer.u8(self.times_crafted);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftResultsDeprecated
    }
}

#[derive(Debug)]
pub struct CreateStackRequestAction {
    pub results_slot: u8,
}

impl CreateStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            results_slot: reader.u8(),
        }
    }
}

impl StackRequestAction for CreateStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.results_slot);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Create
    }
}
