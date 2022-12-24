use std::collections::BTreeMap;
use std::fmt::Debug;

use bytes::Bytes;
use glam::{IVec3, Vec3};
use uuid::Uuid;

use crate::encodable_enum;
use crate::enums::*;
use crate::io::{Reader, Writer, Write, Read};

#[derive(Debug)]
pub struct AbilityData {
    entity_unique_id: i64,
    player_permissions: u8,
    command_permission: u8,
    layers: Vec<AbilityLayer>,
}

impl AbilityData {
    pub fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(self.player_permissions);
        writer.u8(self.command_permission);
        writer.u8(self.layers.len() as u8);
        self.layers.iter().for_each(|layer| layer.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            player_permissions: reader.u8(),
            command_permission: reader.u8(),
            layers: (0..reader.u8()).map(|_| AbilityLayer::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct AbilityLayer {
    layer_type: AbilityLayerType,
    abilities: u32,
    values: u32,
    fly_speed: f32,
    walk_speed: f32,
}

impl AbilityLayer {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.layer_type).unwrap());
        writer.u32(self.abilities);
        writer.u32(self.values);
        writer.f32(self.fly_speed);
        writer.f32(self.walk_speed);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            layer_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            abilities: reader.u32(),
            values: reader.u32(),
            fly_speed: reader.f32(),
            walk_speed: reader.f32(),
        }
    }
}

#[derive(Debug)]
pub struct AchievementAwardedEventData {
    pub achievement_id: i32,
}

impl AchievementAwardedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            achievement_id: reader.var_i32(),
        }
    }
}

impl EventData for AchievementAwardedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.achievement_id);
    }

    fn event_type(&self) -> EventType {
        EventType::AchievementAwarded
    }
}

#[derive(Debug)]
pub struct AgentCommandEventData {
    pub agent_result: i32,
    pub data_value: i32,
    pub command: String,
    pub data_key: String,
    pub output: String,
}

impl AgentCommandEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            agent_result: reader.var_i32(),
            data_value: reader.var_i32(),
            command: reader.string(),
            data_key: reader.string(),
            output: reader.string(),
        }
    }
}

impl EventData for AgentCommandEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.agent_result);
        writer.var_i32(self.data_value);
        writer.string(self.command.as_str());
        writer.string(self.data_key.as_str());
        writer.string(self.output.as_str());
    }

    fn event_type(&self) -> EventType {
        EventType::AgentCommand
    }
}

#[derive(Debug)]
pub struct AgentCreatedEventData {}

impl AgentCreatedEventData {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

impl EventData for AgentCreatedEventData {
    fn write(&self, _: &mut Writer) {}

    fn event_type(&self) -> EventType {
        EventType::AgentCreated
    }
}

#[derive(Debug, Default)]
pub struct Attribute {
    pub value: AttributeValue,
    pub default: f32,
    pub modifiers: Vec<AttributeModifier>,
}

impl Attribute {
    pub fn write(&self, writer: &mut Writer) {
        writer.f32(self.value.min);
        writer.f32(self.value.max);
        writer.f32(self.value.value);
        writer.f32(self.default);
        writer.string(self.value.name.as_str());
        writer.var_u32(self.modifiers.len() as u32);
        self.modifiers.iter().for_each(|modifier| modifier.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        let mut attribute = Self::default();
        attribute.value = AttributeValue {
            min: reader.f32(),
            max: reader.f32(),
            value: reader.f32(),
            ..Default::default()
        };
        attribute.default = reader.f32();
        attribute.value.name = reader.string();
        attribute.modifiers = (0..reader.var_u32()).map(|_| AttributeModifier::read(reader)).collect();

        attribute
    }
}

#[derive(Debug)]
pub struct AttributeModifier {
    pub id: String,
    pub name: String,
    pub amount: f32,
    pub operation: i32,
    pub operand: i32,
    pub serializable: bool,
}

impl AttributeModifier {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.id.as_str());
        writer.string(self.name.as_str());
        writer.f32(self.amount);
        writer.i32(self.operation);
        writer.i32(self.operand);
        writer.bool(self.serializable);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            id: reader.string(),
            name: reader.string(),
            amount: reader.f32(),
            operation: reader.i32(),
            operand: reader.i32(),
            serializable: reader.bool(),
        }
    }
}

#[derive(Debug, Default)]
pub struct AttributeValue {
    pub name: String,
    pub min: f32,
    pub value: f32,
    pub max: f32,
}

impl AttributeValue {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.f32(self.min);
        writer.f32(self.value);
        writer.f32(self.max);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            min: reader.f32(),
            value: reader.f32(),
            max: reader.f32(),
        }
    }
}

#[derive(Debug)]
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
            ingredients: (0..reader.var_u32()).map(|_| ItemDescriptorCount::read(reader)).collect(),
        }
    }
}

impl StackRequestAction for AutoCraftRecipeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u32(self.recipe_network_id);
        writer.u8(self.times_crafted);
        writer.var_u32(self.ingredients.len() as u32);
        self.ingredients.iter().for_each(|ingredient| ingredient.write(writer));
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::CraftRecipeAuto
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for BeaconPaymentStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.primary_effect);
        writer.var_i32(self.secondary_effect);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::BeaconPayment
    }
}

#[derive(Debug)]
pub struct BehaviourPackInfo {
    pub uuid: String,
    pub version: String,
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
}

impl BehaviourPackInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.string(self.version.as_str());
        writer.u64(self.size);
        writer.string(self.content_key.as_str());
        writer.string(self.sub_pack_name.as_str());
        writer.string(self.content_identity.as_str());
        writer.bool(self.has_scripts);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            version: reader.string(),
            size: reader.u64(),
            content_key: reader.string(),
            sub_pack_name: reader.string(),
            content_identity: reader.string(),
            has_scripts: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct BellUsedEventData {
    pub item_id: i32,
}

impl BellUsedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_id: reader.var_i32(),
        }
    }
}

impl EventData for BellUsedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.item_id);
    }

    fn event_type(&self) -> EventType {
        EventType::BellUsed
    }
}

#[derive(Debug)]
pub struct BlockChangeEntry {
    pub block_pos: BlockPos,
    pub block_runtime_id: u32,
    pub flags: u32,
    pub synced_update_entity_unique_id: u64,
    pub synced_update_type: u32,
}

impl BlockChangeEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.block_pos);
        writer.var_u32(self.block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u64(self.synced_update_entity_unique_id);
        writer.var_u32(self.synced_update_type);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_pos: reader.block_pos(),
            block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            synced_update_entity_unique_id: reader.var_u64(),
            synced_update_type: reader.var_u32(),
        }
    }
}

#[derive(Debug)]
pub struct BlockEntry {
    pub name: String,
    //pub properties: dyn Any, // TODO: NBT
}

impl BlockEntry {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        // TODO: NBT (properties)
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            // properties: {
            //     // TODO: NBT
            // },
        }
    }
}

pub type BlockPos = IVec3;

#[derive(Debug)]
pub struct BossKilledEventData {
    pub boss_entity_unique_id: i64,
    pub player_party_size: i32,
    pub interaction_entity_type: i32,
}

impl BossKilledEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            boss_entity_unique_id: reader.var_i64(),
            player_party_size: reader.var_i32(),
            interaction_entity_type: reader.var_i32(),
        }
    }
}

impl EventData for BossKilledEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.boss_entity_unique_id);
        writer.var_i32(self.player_party_size);
        writer.var_i32(self.interaction_entity_type);
    }

    fn event_type(&self) -> EventType {
        EventType::BossKilled
    }
}

#[derive(Debug)]
pub struct CacheBlob {
    pub hash: u64,
    pub payload: Bytes,
}

impl CacheBlob {
    pub fn write(&self, writer: &mut Writer) {
        writer.u64(self.hash);
        writer.byte_slice(&self.payload);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            hash: reader.u64(),
            payload: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct CauldronInteractEventData {
    pub block_interaction_type: i32,
    pub item_id: i32,
}

impl CauldronInteractEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_interaction_type: reader.var_i32(),
            item_id: reader.var_i32(),
        }
    }
}

impl EventData for CauldronInteractEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.block_interaction_type);
        writer.var_i32(self.item_id);
    }

    fn event_type(&self) -> EventType {
        EventType::CauldronInteract
    }
}

#[derive(Debug)]
pub struct CauldronUsedEventData {
    pub potion_id: i32,
    pub colour: i32,
    pub fill_level: i32,
}

impl CauldronUsedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            potion_id: reader.var_i32(),
            colour: reader.var_i32(),
            fill_level: reader.var_i32(),
        }
    }
}

impl EventData for CauldronUsedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.potion_id);
        writer.var_i32(self.colour);
        writer.var_i32(self.fill_level);
    }

    fn event_type(&self) -> EventType {
        EventType::CauldronUsed
    }
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub flags: u16,
    pub permission_level: u8,
    pub aliases: Vec<String>,
    pub overloads: Vec<CommandOverload>,
}

impl Command {
    pub fn write(&self, writer: &mut Writer) {
        // writer.string(self.name.as_str());
        // writer.string(self.description.as_str());
        // writer.u16(self.flags);
        // writer.u8(self.permission_level);
        // writer.write_TODO(self.LEN);
        // writer.write_String(self.aliases);
        // writer.write_TODO(self.LEN);
        // writer.write_CommandOverload(self.overloads);
        todo!()
    }

    pub fn read(reader: &mut Reader) -> Self {
        // Self {
        //     name: reader.string(),
        //     description: reader.string(),
        //     flags: reader.u16(),
        //     permission_level: reader.u8(),
        //     LEN: reader.read_TODO(),
        //     aliases: reader.read_String(),
        //     LEN: reader.read_TODO(),
        //     overloads: reader.read_CommandOverload(),
        // }
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CommandEnum {
    pub enum_type: String,
    pub options: Vec<String>,
    pub dynamic: bool,
}

impl CommandEnum {
    pub fn write(&self, writer: &mut Writer, value_indices: BTreeMap<String, usize>) {
        writer.string(self.enum_type.as_str());
        writer.var_u32(self.options.len() as u32);
        if self.dynamic {
            self.options.iter().for_each(|option| writer.string(option.as_str()));
        } else {
            let len = value_indices.len();
            if len <= u8::MAX as usize {
                self.options.iter().for_each(|option| writer.u8(*value_indices.get(option).unwrap() as u8));
            } else if len <= u16::MAX as usize {
                self.options.iter().for_each(|option| writer.u16(*value_indices.get(option).unwrap() as u16));
            } else {
                self.options.iter().for_each(|option| writer.u32(*value_indices.get(option).unwrap() as u32));
            }
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let command_enum = Self {
            enum_type: reader.string(),
            ..Default::default()
        };
        // TODO: READING

        command_enum
    }
}

#[derive(Debug)]
pub struct CommandEnumConstraint {
    pub enum_option: String,
    pub enum_name: String,
    pub constraints: Bytes,
}

impl CommandEnumConstraint {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.enum_option.as_str());
        writer.string(self.enum_name.as_str());
        writer.byte_slice(&self.constraints);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            enum_option: reader.string(),
            enum_name: reader.string(),
            constraints: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct CommandOrigin {
    pub origin: CommandOriginType,
    pub uuid: Uuid,
    pub request_id: String,
    pub player_unique_id: i64,
}

impl CommandOrigin {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(num::ToPrimitive::to_u32(&self.origin).unwrap());
        writer.uuid(self.uuid);
        writer.string(self.request_id.as_str());
        writer.i64(self.player_unique_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            origin: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
            uuid: reader.uuid(),
            request_id: reader.string(),
            player_unique_id: reader.i64(),
        }
    }
}

#[derive(Debug)]
pub struct CommandOutputMessage {
    pub success: bool,
    pub message: String,
    pub parameters: Vec<String>,
}

impl CommandOutputMessage {
    pub fn write(&self, writer: &mut Writer) {
        writer.bool(self.success);
        writer.string(self.message.as_str());
        writer.var_u32(self.parameters.len() as u32);
        self.parameters.iter().for_each(|parameter| writer.string(parameter.as_str()));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            success: reader.bool(),
            message: reader.string(),
            parameters: (0..reader.var_u32()).map(|_| reader.string()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct CommandOverload {
    pub parameters: Vec<CommandParameter>,
}

impl CommandOverload {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.parameters.len() as u32);
        self.parameters.iter().for_each(|parameter| parameter.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            parameters: (0..reader.var_u32()).map(|_| CommandParameter::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct CommandParameter {
    pub name: String,
    pub parameter_type: u32,
    pub optional: bool,
    pub options: u8,
    pub command_enum: CommandEnum,
    pub suffix: String,
}

impl CommandParameter {
    pub fn write(&self, writer: &mut Writer) {
        // if self.command_enum.dynamic {
        //     self.parameter_type = CommandArg::SoftEnum | CommandArg::Valid |
        // }
        writer.string(self.name.as_str());
        writer.u32(self.parameter_type);
        writer.bool(self.optional);
        writer.u8(self.options);
        todo!()
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            parameter_type: reader.u32(),
            optional: reader.bool(),
            options: reader.u8(),
            command_enum: CommandEnum::read(reader),
            suffix: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct ComposterInteractEventData {
    pub block_interaction_type: i32,
    pub item_id: i32,
}

impl ComposterInteractEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_interaction_type: reader.var_i32(),
            item_id: reader.var_i32(),
        }
    }
}

impl EventData for ComposterInteractEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.block_interaction_type);
        writer.var_i32(self.item_id);
    }

    fn event_type(&self) -> EventType {
        EventType::ComposterInteract
    }
}

#[derive(Debug)]
pub struct ContainerDataKey(pub i32);

impl Into<ContainerDataKey> for ContainerDataFurnace {
    fn into(self) -> ContainerDataKey {
        ContainerDataKey(num::ToPrimitive::to_i32(&self).unwrap())
    }
}

impl Into<ContainerDataKey> for ContainerDataBrewingStand {
    fn into(self) -> ContainerDataKey {
        ContainerDataKey(num::ToPrimitive::to_i32(&self).unwrap())
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

#[derive(Debug)]
pub struct CreativeItem {
    pub creative_item_network_id: u32,
    pub item: ItemStack,
}

impl CreativeItem {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.creative_item_network_id);
        self.item.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            creative_item_network_id: reader.var_u32(),
            item: ItemStack::read(reader),
        }
    }
}

#[derive(Debug)]
pub struct DefaultItemDescriptor {
    network_id: i16,
    metadata: i16,
}

impl DefaultItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.i16(),
            metadata: reader.i16(),
        }
    }
}

impl ItemDescriptor for DefaultItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.network_id);
        writer.i16(self.metadata);
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Deferred
    }
}

#[derive(Debug)]
pub struct DeferredItemDescriptor {
    name: String,
    metadata: i16,
}

impl DeferredItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            metadata: reader.i16(),
        }
    }
}

impl ItemDescriptor for DeferredItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.i16(self.metadata);
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Deferred
    }
}

#[derive(Debug)]
pub struct DimensionDefinition {
    name: String,
    range: [i32; 2],
    generator: i32,
}

impl DimensionDefinition {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.var_i32(self.range[0]);
        writer.var_i32(self.range[1]);
        writer.var_i32(self.generator);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            range: [reader.var_i32(), reader.var_i32()],
            generator: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for DestroyStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Destroy
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for DropStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        writer.bool(self.randomly);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Drop
    }
}

#[derive(Debug)]
pub struct EducationExternalLinkSettings {
    pub url: String,
    pub display_name: String,
}

impl EducationExternalLinkSettings {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.url.as_str());
        writer.string(self.display_name.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            url: reader.string(),
            display_name: reader.string(),
        }
    }
}

impl Write for EducationExternalLinkSettings {
    fn write(&self, writer: &mut Writer) {
        self.write(writer)
    }
}

impl Read<EducationExternalLinkSettings> for EducationExternalLinkSettings {
    fn read(reader: &mut Reader) -> EducationExternalLinkSettings {
        EducationExternalLinkSettings::read(reader)
    }
}

#[derive(Debug)]
pub struct EducationSharedResourceURI {
    pub button_name: String,
    pub link_uri: String,
}

impl EducationSharedResourceURI {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.button_name.as_str());
        writer.string(self.link_uri.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            button_name: reader.string(),
            link_uri: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct EnchantmentInstance {
    pub enchantment_type: u8,
    pub level: u8,
}

impl EnchantmentInstance {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.enchantment_type);
        writer.u8(self.level);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            enchantment_type: reader.u8(),
            level: reader.u8(),
        }
    }
}

#[derive(Debug)]
pub struct EnchantmentOption {
    pub cost: u32,
    pub enchantments: ItemEnchantments,
    pub name: String,
    pub recipe_network_id: u32,
}

impl EnchantmentOption {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.cost);
        self.enchantments.write(writer);
        writer.string(self.name.as_str());
        writer.var_u32(self.recipe_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            cost: reader.var_u32(),
            enchantments: ItemEnchantments::read(reader),
            name: reader.string(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

#[derive(Debug)]
pub struct EntityDefinitionTriggerEventData {
    pub event_name: String,
}

impl EntityDefinitionTriggerEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            event_name: reader.string(),
        }
    }
}

impl EventData for EntityDefinitionTriggerEventData {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.event_name.as_str());
    }

    fn event_type(&self) -> EventType {
        EventType::EntityDefinitionTrigger
    }
}

#[derive(Debug)]
pub struct EntityInteractEventData {
    pub interaction_type: i32,
    pub interaction_entity_type: i32,
    pub entity_variant: i32,
    pub entity_colour: u8,
}

impl EntityInteractEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            interaction_type: reader.var_i32(),
            interaction_entity_type: reader.var_i32(),
            entity_variant: reader.var_i32(),
            entity_colour: reader.u8(),
        }
    }
}

impl EventData for EntityInteractEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.interaction_type);
        writer.var_i32(self.interaction_entity_type);
        writer.var_i32(self.entity_variant);
        writer.u8(self.entity_colour);
    }

    fn event_type(&self) -> EventType {
        EventType::EntityInteract
    }
}

#[derive(Debug)]
pub struct EntityLink {
    pub ridden_entity_unique_id: i64,
    pub rider_entity_unique_id: i64,
    pub link_type: EntityLinkType,
    pub immediate: bool,
    pub rider_initiated: bool,
}

impl EntityLink {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.ridden_entity_unique_id);
        writer.var_i64(self.rider_entity_unique_id);
        writer.u8(num::ToPrimitive::to_u8(&self.link_type).unwrap());
        writer.bool(self.immediate);
        writer.bool(self.rider_initiated);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            ridden_entity_unique_id: reader.i64(),
            rider_entity_unique_id: reader.i64(),
            link_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            immediate: reader.bool(),
            rider_initiated: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct ExperimentData {
    pub name: String,
    pub enabled: bool,
}

impl ExperimentData {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.enabled);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            enabled: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct ExtractHoneyEventData {}

impl ExtractHoneyEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

impl EventData for ExtractHoneyEventData {
    fn write(&self, _: &mut Writer) {}

    fn event_type(&self) -> EventType {
        EventType::ExtractHoney
    }
}

pub trait EventData: Debug {
    fn write(&self, writer: &mut Writer);
    fn event_type(&self) -> EventType;
}

encodable_enum!(
    #[derive(Debug)]
    pub enum EventData2 {
        AchievementAwardedEventData = 0,
        EntityInteractEventData = 1,
        PortalBuiltEventData = 2,
        PortalUsedEventData = 3,
        MobKilledEventData = 4,
        CauldronUsedEventData = 5,
        PlayerDiedEventData = 6,
        BossKilledEventData = 7,
        AgentCommandEventData = 8,
        AgentCreatedEventData = 9,
        PatternRemovedEventData = 10,
        SlashCommandExecutedEventData = 11,
        FishBucketedEventData = 12,
        MobBornEventData = 13,
        PetDiedEventData = 14,
        CauldronInteractEventData = 15,
        ComposterInteractEventData = 16,
        BellUsedEventData = 17,
        EntityDefinitionTriggerEventData = 18,
        RaidUpdateEventData = 19,
        MovementAnomalyEventData = 20,
        MovementCorrectedEventData = 21,
        //ExtractHoneyEventData = 22, todo
        //TargetBlockHitEventData = 23, todo
        //PiglinBarterEventData = 24, todo
        PlayerWaxedOrUnwaxedCopperEventData = 25,
        //CodeBuilderRuntimeActionEventData = 26, todo
        //CodeBuilderScoreboardEventData = 27, todo
        //StriderRiddenInLavaInOverworldEventData = 28, todo
        SneakCloseToSculkSensorEventData = 29,
    }
);

#[derive(Debug)]
pub struct FishBucketedEventData {
    pub pattern: i32,
    pub preset: i32,
    pub bucketed_entity_type: i32,
    pub release: bool,
}

impl FishBucketedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            pattern: reader.var_i32(),
            preset: reader.var_i32(),
            bucketed_entity_type: reader.var_i32(),
            release: reader.bool(),
        }
    }
}

impl EventData for FishBucketedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.pattern);
        writer.var_i32(self.preset);
        writer.var_i32(self.bucketed_entity_type);
        writer.bool(self.release);
    }

    fn event_type(&self) -> EventType {
        EventType::FishBucketed
    }
}

#[derive(Debug)]
pub struct FurnaceDataRecipe {
    pub furnace_recipe: FurnaceRecipe,
}

impl FurnaceDataRecipe {
    pub fn write(&self, writer: &mut Writer) {
        self.furnace_recipe.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            furnace_recipe: FurnaceRecipe::read(reader),
        }
    }
}

#[derive(Debug)]
pub struct FurnaceRecipe {
    pub network_id: i32,
    pub output: ItemStack,
    pub block: String,
}

impl FurnaceRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.network_id);
        self.output.write(writer);
        writer.string(self.block.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.var_i32(),
            output: ItemStack::read(reader),
            block: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct GameRule {
    pub name: String,
    pub can_be_modified_by_player: bool,
    //pub value: dyn Any, // TODO
}

impl GameRule {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.can_be_modified_by_player);
        //writer.write_TODO(self.value);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            can_be_modified_by_player: reader.bool(),
            //value: reader.read_TODO(),
        }
    }
}

#[derive(Debug)]
pub struct GenerationFeature {
    name: String,
    json: Bytes,
}

impl GenerationFeature {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.byte_slice(&self.json);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            json: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct InvalidItemDescriptor {}

impl InvalidItemDescriptor {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

impl ItemDescriptor for InvalidItemDescriptor {
    fn write(&self, _: &mut Writer) {}

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::Invalid
    }
}

#[derive(Debug)]
pub struct InventoryAction {
    pub source_type: InventoryActionSource,
    pub window: Window,
    pub source_flags: u32,
    pub inventory_slot: u32,
    pub old_item: ItemInstance,
    pub new_item: ItemInstance,
}

impl InventoryAction {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(num::ToPrimitive::to_u32(&self.source_type).unwrap());
        match self.source_type {
            InventoryActionSource::Container | InventoryActionSource::TODO => {
                writer.var_i32(num::ToPrimitive::to_i32(&self.window).unwrap());
            }
            InventoryActionSource::World => {
                writer.var_u32(self.source_flags);
            }
            _ => {}
        }
        writer.var_u32(self.inventory_slot);
        self.old_item.write(writer);
        self.new_item.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        let source_type: InventoryActionSource = num::FromPrimitive::from_u32(reader.var_u32()).unwrap();
        Self {
            source_type: source_type.clone(),
            window: if source_type == InventoryActionSource::Container || source_type == InventoryActionSource::TODO {
                num::FromPrimitive::from_i32(reader.var_i32()).unwrap()
            } else {
                Window::Inventory
            },
            source_flags: if source_type == InventoryActionSource::World {
                reader.var_u32()
            } else {
                0
            },
            inventory_slot: reader.var_u32(),
            old_item: ItemInstance::read(reader),
            new_item: ItemInstance::read(reader),
        }
    }
}

impl Default for InventoryAction {
    fn default() -> Self {
        Self {
            source_type: InventoryActionSource::Container,
            window: Window::Inventory,
            source_flags: 0,
            inventory_slot: 0,
            old_item: ItemInstance::default(),
            new_item: ItemInstance::default(),
        }
    }
}

pub trait InventoryTransactionData: Debug {
    fn write(&self, writer: &mut Writer);
    fn transaction_type(&self) -> InventoryTransactionType;
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
            //writer.var_u32(num::ToPrimitive::to_u32(&action.action_type()).unwrap());
            //action.write(writer);
        });
        writer.var_u32(self.filter_strings.len() as u32);
        self.filter_strings.iter().for_each(|filter_string| writer.string(filter_string.as_str()));
        writer.i32(self.filter_cause);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            request_id: reader.var_i32(),
            actions: (0..reader.var_u32()).map(|_| {
                /*let action_type: StackRequestActionType = num::FromPrimitive::from_u32(reader.var_u32()).unwrap();
                match action_type {
                    StackRequestActionType::Take => Box::from(TakeStackRequestAction::read(reader)),
                    StackRequestActionType::Place => Box::from(PlaceStackRequestAction::read(reader)),
                    StackRequestActionType::Swap => Box::from(SwapStackRequestAction::read(reader)),
                    StackRequestActionType::Drop => Box::from(DropStackRequestAction::read(reader)),
                    StackRequestActionType::Destroy => Box::from(DestroyStackRequestAction::read(reader)),
                    StackRequestActionType::Consume => Box::from(ConsumeStackRequestAction::read(reader)),
                    StackRequestActionType::Create => Box::from(CreateStackRequestAction::read(reader)),
                    StackRequestActionType::PlaceInContainer => Box::from(PlaceInContainerStackRequestAction::read(reader)),
                    StackRequestActionType::TakeOutContainer => Box::from(TakeOutContainerStackRequestAction::read(reader)),
                    StackRequestActionType::LabTableCombine => Box::from(LabTableCombineStackRequestAction::read(reader)),
                    StackRequestActionType::BeaconPayment => Box::from(BeaconPaymentStackRequestAction::read(reader)),
                    StackRequestActionType::MineBlock => Box::from(MineBlockStackRequestAction::read(reader)),
                    StackRequestActionType::CraftRecipe => Box::from(CraftRecipeStackRequestAction::read(reader)),
                    StackRequestActionType::CraftRecipeAuto => Box::from(AutoCraftRecipeStackRequestAction::read(reader)),
                    StackRequestActionType::CraftCreative => Box::from(CraftCreativeStackRequestAction::read(reader)),
                    StackRequestActionType::CraftRecipeOptional => Box::from(CraftRecipeOptionalStackRequestAction::read(reader)),
                    StackRequestActionType::CraftGrindstone => Box::from(CraftGrindstoneRecipeStackRequestAction::read(reader)),
                    StackRequestActionType::CraftLoom => Box::from(CraftLoomRecipeStackRequestAction::read(reader)),
                    StackRequestActionType::CraftNonImplementedDeprecated => Box::from(CraftNonImplementedStackRequestAction::read(reader)),
                    StackRequestActionType::CraftResultsDeprecated => Box::from(CraftResultsDeprecatedStackRequestAction::read(reader)),
                }*/
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
pub struct LegacySetItemSlot {
    pub container_id: u8,
    pub slots: Bytes,
}

impl LegacySetItemSlot {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.container_id);
        writer.byte_slice(&self.slots);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            container_id: reader.u8(),
            slots: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct MapDecoration {
    pub decoration_type: u8,
    pub rotation: u8,
    pub x: u8,
    pub y: u8,
    pub label: String,
    pub colour: RGBA,
}

impl MapDecoration {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.decoration_type);
        writer.u8(self.rotation);
        writer.u8(self.x);
        writer.u8(self.y);
        writer.string(self.label.as_str());
        self.colour.write_var(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            decoration_type: reader.u8(),
            rotation: reader.u8(),
            x: reader.u8(),
            y: reader.u8(),
            label: reader.string(),
            colour: RGBA::read_var(reader),
        }
    }
}

#[derive(Debug)]
pub struct MapTrackedObject {
    pub object_type: MapObjectType,
    pub entity_unique_id: i64,
    pub block_position: IVec3,
}

impl MapTrackedObject {
    pub fn write(&self, writer: &mut Writer) {
        writer.i32(num::ToPrimitive::to_i32(&self.object_type).unwrap());
        writer.i64(self.entity_unique_id);
        writer.u_block_pos(self.block_position);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            object_type: num::FromPrimitive::from_i32(reader.i32()).unwrap(),
            entity_unique_id: reader.i64(),
            block_position: reader.u_block_pos(),
        }
    }
}

#[derive(Debug)]
pub struct MaterialReducer {
    pub network_id: i32,
    pub metadata_value: u32,
    pub outputs: Vec<MaterialReducerOutput>,
}

impl MaterialReducer {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32((self.network_id << 16) | (self.metadata_value as i32));
        writer.var_u32(self.outputs.len() as u32);
        self.outputs.iter().for_each(|output| output.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.var_i32() >> 16,
            metadata_value: (reader.var_i32() & 0xFFFF) as u32,
            outputs: (0..reader.var_u32()).map(|_| MaterialReducerOutput::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct MaterialReducerOutput {
    pub network_id: i32,
    pub count: i32,
}

impl MaterialReducerOutput {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.network_id);
        writer.var_i32(self.count);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.var_i32(),
            count: reader.var_i32(),
        }
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
pub struct MismatchTransactionData {}

impl MismatchTransactionData {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

impl InventoryTransactionData for MismatchTransactionData {
    fn write(&self, _: &mut Writer) {}

    fn transaction_type(&self) -> InventoryTransactionType {
        InventoryTransactionType::Mismatch
    }
}

#[derive(Debug)]
pub struct MobBornEventData {
    pub entity_type: i32,
    pub variant: i32,
    pub colour: u8,
}

impl MobBornEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            entity_type: reader.var_i32(),
            variant: reader.var_i32(),
            colour: reader.u8(),
        }
    }
}

impl EventData for MobBornEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
        writer.var_i32(self.variant);
        writer.u8(self.colour);
    }

    fn event_type(&self) -> EventType {
        EventType::AchievementAwarded
    }
}

#[derive(Debug)]
pub struct MobKilledEventData {
    pub killer_entity_unique_id: i64,
    pub victim_entity_unique_id: i64,
    pub killer_entity_type: i32,
    pub entity_damage_cause: i32,
    pub villager_trade_tier: i32,
    pub villager_display_name: String,
}

impl MobKilledEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            killer_entity_unique_id: reader.var_i64(),
            victim_entity_unique_id: reader.var_i64(),
            killer_entity_type: reader.var_i32(),
            entity_damage_cause: reader.var_i32(),
            villager_trade_tier: reader.var_i32(),
            villager_display_name: reader.string(),
        }
    }
}

impl EventData for MobKilledEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.killer_entity_unique_id);
        writer.var_i64(self.victim_entity_unique_id);
        writer.var_i32(self.killer_entity_type);
        writer.var_i32(self.entity_damage_cause);
        writer.var_i32(self.villager_trade_tier);
        writer.string(self.villager_display_name.as_str());
    }

    fn event_type(&self) -> EventType {
        EventType::MobKilled
    }
}

#[derive(Debug)]
pub struct MoLangItemDescriptor {
    expression: String,
    version: u8,
}

impl MoLangItemDescriptor {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            expression: reader.string(),
            version: reader.u8(),
        }
    }
}

impl ItemDescriptor for MoLangItemDescriptor {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.expression.as_str());
        writer.u8(self.version);
    }

    fn descriptor_type(&self) -> ItemDescriptorType {
        ItemDescriptorType::MoLang
    }
}

#[derive(Debug)]
pub struct MovementAnomalyEventData {
    pub event_type: u8,
    pub cheating_score: f32,
    pub average_position_delta: f32,
    pub total_position_delta: f32,
    pub min_position_delta: f32,
    pub max_position_delta: f32,
}

impl MovementAnomalyEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            event_type: reader.u8(),
            cheating_score: reader.f32(),
            average_position_delta: reader.f32(),
            total_position_delta: reader.f32(),
            min_position_delta: reader.f32(),
            max_position_delta: reader.f32(),
        }
    }
}

impl EventData for MovementAnomalyEventData {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.event_type);
        writer.f32(self.cheating_score);
        writer.f32(self.average_position_delta);
        writer.f32(self.total_position_delta);
        writer.f32(self.min_position_delta);
        writer.f32(self.max_position_delta);
    }

    fn event_type(&self) -> EventType {
        EventType::MovementAnomaly
    }
}

#[derive(Debug)]
pub struct MovementCorrectedEventData {
    pub position_delta: f32,
    pub cheating_score: f32,
    pub score_threshold: f32,
    pub distance_threshold: f32,
    pub duration_threshold: i32,
}

impl MovementCorrectedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            position_delta: reader.f32(),
            cheating_score: reader.f32(),
            score_threshold: reader.f32(),
            distance_threshold: reader.f32(),
            duration_threshold: reader.var_i32(),
        }
    }
}

impl EventData for MovementCorrectedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.position_delta);
        writer.f32(self.cheating_score);
        writer.f32(self.score_threshold);
        writer.f32(self.distance_threshold);
        writer.var_i32(self.duration_threshold);
    }

    fn event_type(&self) -> EventType {
        EventType::MovementCorrected
    }
}

#[derive(Debug)]
pub struct MultiRecipe {
    pub uuid: Uuid,
    pub recipe_network_id: u32,
}

impl MultiRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.uuid(self.uuid);
        writer.var_u32(self.recipe_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.uuid(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

#[derive(Debug)]
pub struct NormalTransactionData {}

impl NormalTransactionData {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

impl InventoryTransactionData for NormalTransactionData {
    fn write(&self, _: &mut Writer) {}

    fn transaction_type(&self) -> InventoryTransactionType {
        InventoryTransactionType::Normal
    }
}

#[derive(Debug)]
pub struct PatternRemovedEventData {
    pub item_id: i32,
    pub aux_value: i32,
    pub patterns_size: i32,
    pub pattern_index: i32,
    pub pattern_colour: i32,
}

impl PatternRemovedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_id: reader.var_i32(),
            aux_value: reader.var_i32(),
            patterns_size: reader.var_i32(),
            pattern_index: reader.var_i32(),
            pattern_colour: reader.var_i32(),
        }
    }
}

impl EventData for PatternRemovedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.item_id);
        writer.var_i32(self.aux_value);
        writer.var_i32(self.patterns_size);
        writer.var_i32(self.pattern_index);
        writer.var_i32(self.pattern_colour);
    }

    fn event_type(&self) -> EventType {
        EventType::PatternRemoved
    }
}

#[derive(Debug)]
pub struct PersonaPiece {
    pub piece_id: String,
    pub piece_type: String,
    pub pack_id: String,
    pub default: bool,
    pub product_id: String,
}

impl PersonaPiece {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.piece_id.as_str());
        writer.string(self.piece_type.as_str());
        writer.string(self.pack_id.as_str());
        writer.bool(self.default);
        writer.string(self.product_id.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            piece_id: reader.string(),
            piece_type: reader.string(),
            pack_id: reader.string(),
            default: reader.bool(),
            product_id: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct PersonaPieceTintColour {
    pub piece_type: String,
    pub colours: Vec<String>,
}

impl PersonaPieceTintColour {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.piece_type.as_str());
        writer.u32(self.colours.len() as u32);
        self.colours.iter().for_each(|colour| writer.string(colour.as_str()));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            piece_type: reader.string(),
            colours: (0..reader.u32()).map(|_| reader.string()).collect::<Vec<String>>(),
        }
    }
}

#[derive(Debug)]
pub struct PetDiedEventData {
    pub killed_by_owner: bool,
    pub killer_entity_unique_id: i64,
    pub pet_entity_unique_id: i64,
    pub entity_damage_cause: i32,
    pub pet_entity_type: i32,
}

impl PetDiedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            killed_by_owner: reader.bool(),
            killer_entity_unique_id: reader.var_i64(),
            pet_entity_unique_id: reader.var_i64(),
            entity_damage_cause: reader.var_i32(),
            pet_entity_type: reader.var_i32(),
        }
    }
}

impl EventData for PetDiedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.killed_by_owner);
        writer.var_i64(self.killer_entity_unique_id);
        writer.var_i64(self.pet_entity_unique_id);
        writer.var_i32(self.entity_damage_cause);
        writer.var_i32(self.pet_entity_type);
    }

    fn event_type(&self) -> EventType {
        EventType::PetDied
    }
}

#[derive(Debug)]
pub struct PixelRequest {
    colour: RGBA,
    index: u16,
}

impl PixelRequest {
    pub fn write(&self, writer: &mut Writer) {
        self.colour.write(writer);
        writer.u16(self.index);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            colour: RGBA::read(reader),
            index: reader.u16(),
        }
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for PlaceInContainerStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::PlaceInContainer
    }
}

#[derive(Debug)]
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
}

impl StackRequestAction for PlaceStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Place
    }
}

#[derive(Debug)]
pub struct PlayerBlockAction {
    pub action: PlayerActionType,
    pub block_pos: BlockPos,
    pub face: i32,
}

impl PlayerBlockAction {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.action).unwrap());
        match self.action {
            PlayerActionType::StartBreak | PlayerActionType::AbortBreak | PlayerActionType::CrackBreak | PlayerActionType::PredictDestroyBlock | PlayerActionType::ContinueDestroyBlock => {
                writer.block_pos(self.block_pos);
                writer.var_i32(self.face);
            }
            _ => {}
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let mut action = Self {
            action: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            block_pos: BlockPos::default(),
            face: 0,
        };
        match action.action {
            PlayerActionType::StartBreak | PlayerActionType::AbortBreak | PlayerActionType::CrackBreak | PlayerActionType::PredictDestroyBlock | PlayerActionType::ContinueDestroyBlock => {
                action.block_pos = reader.block_pos();
                action.face = reader.var_i32();
            }
            _ => {}
        }

        action
    }
}

#[derive(Debug)]
pub struct PlayerDiedEventData {
    pub attacker_entity_id: i32,
    pub attacker_variant: i32,
    pub entity_damage_cause: i32,
    pub in_raid: bool,
}

impl PlayerDiedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            attacker_entity_id: reader.var_i32(),
            attacker_variant: reader.var_i32(),
            entity_damage_cause: reader.var_i32(),
            in_raid: reader.bool(),
        }
    }
}

impl EventData for PlayerDiedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.attacker_entity_id);
        writer.var_i32(self.attacker_variant);
        writer.var_i32(self.entity_damage_cause);
        writer.bool(self.in_raid);
    }

    fn event_type(&self) -> EventType {
        EventType::PlayerDied
    }
}

#[derive(Debug, Default)]
pub struct PlayerListEntry {
    pub uuid: Uuid,
    pub entity_unique_id: i64,
    pub username: String,
    pub xuid: String,
    pub platform_chat_id: String,
    pub build_platform: i32,
    pub skin: Skin,
    pub teacher: bool,
    pub host: bool,
}

impl PlayerListEntry {
    pub fn write(&self, writer: &mut Writer, action: PlayerListAction) {
        writer.uuid(self.uuid);
        if action == PlayerListAction::Add {
            writer.var_i64(self.entity_unique_id);
            writer.string(self.username.as_str());
            writer.string(self.xuid.as_str());
            writer.string(self.platform_chat_id.as_str());
            writer.i32(self.build_platform);
            self.skin.write(writer);
            writer.bool(self.teacher);
            writer.bool(self.host);
        }
    }

    pub fn read(reader: &mut Reader, action: PlayerListAction) -> Self {
        let mut entry = Self {
            uuid: reader.uuid(),
            ..Default::default()
        };
        if action == PlayerListAction::Add {
            entry.entity_unique_id = reader.var_i64();
            entry.username = reader.string();
            entry.xuid = reader.string();
            entry.platform_chat_id = reader.string();
            entry.build_platform = reader.i32();
            entry.skin = Skin::read(reader);
            entry.teacher = reader.bool();
            entry.host = reader.bool();
        }

        entry
    }
}

#[derive(Debug)]
pub struct PlayerMovementSettings {
    pub movement_type: i32,
    pub rewind_history_size: i32,
    pub server_authoritative_block_breaking: bool,
}

impl PlayerMovementSettings {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.movement_type);
        writer.var_i32(self.rewind_history_size);
        writer.bool(self.server_authoritative_block_breaking);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            movement_type: reader.var_i32(),
            rewind_history_size: reader.var_i32(),
            server_authoritative_block_breaking: reader.bool(),
        }
    }
}


#[derive(Debug)]
pub struct PlayerWaxedOrUnwaxedCopperEventData {}

impl PlayerWaxedOrUnwaxedCopperEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

impl EventData for PlayerWaxedOrUnwaxedCopperEventData {
    fn write(&self, _: &mut Writer) {}

    fn event_type(&self) -> EventType {
        EventType::ExtractHoney
    }
}

#[derive(Debug)]
pub struct PortalBuiltEventData {
    pub dimension_id: i32,
}

impl PortalBuiltEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            dimension_id: reader.var_i32(),
        }
    }
}

impl EventData for PortalBuiltEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension_id);
    }

    fn event_type(&self) -> EventType {
        EventType::PortalBuilt
    }
}

#[derive(Debug)]
pub struct PortalUsedEventData {
    pub from_dimension_id: i32,
    pub to_dimension_id: i32,
}

impl PortalUsedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            from_dimension_id: reader.var_i32(),
            to_dimension_id: reader.var_i32(),
        }
    }
}

impl EventData for PortalUsedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.from_dimension_id);
        writer.var_i32(self.to_dimension_id);
    }

    fn event_type(&self) -> EventType {
        EventType::PortalUsed
    }
}

#[derive(Debug)]
pub struct PotionContainerChangeRecipe {
    pub input_item_id: i32,
    pub reagent_item_id: i32,
    pub output_item_id: i32,
}

impl PotionContainerChangeRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.input_item_id);
        writer.var_i32(self.reagent_item_id);
        writer.var_i32(self.output_item_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            input_item_id: reader.var_i32(),
            reagent_item_id: reader.var_i32(),
            output_item_id: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct PotionRecipe {
    pub input_potion_id: i32,
    pub input_potion_metadata: i32,
    pub reagent_item_id: i32,
    pub reagent_item_metadata: i32,
    pub output_potion_id: i32,
    pub output_potion_metadata: i32,
}

impl PotionRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.input_potion_id);
        writer.var_i32(self.input_potion_metadata);
        writer.var_i32(self.reagent_item_id);
        writer.var_i32(self.reagent_item_metadata);
        writer.var_i32(self.output_potion_id);
        writer.var_i32(self.output_potion_metadata);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            input_potion_id: reader.var_i32(),
            input_potion_metadata: reader.var_i32(),
            reagent_item_id: reader.var_i32(),
            reagent_item_metadata: reader.var_i32(),
            output_potion_id: reader.var_i32(),
            output_potion_metadata: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    pub fn write(&self, writer: &mut Writer) {
        writer.u32((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16) | ((self.a as u32) << 24));
    }

    pub fn write_var(&self, writer: &mut Writer) {
        writer.var_u32((self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16) | ((self.a as u32) << 24));
    }

    pub fn read(reader: &mut Reader) -> Self {
        let value = reader.u32();
        Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }

    pub fn read_var(reader: &mut Reader) -> Self {
        let value = reader.var_u32();
        Self {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }
}

#[derive(Debug)]
pub struct RaidUpdateEventData {
    pub current_raid_wave: i32,
    pub total_raid_waves: i32,
    pub won_raid: bool,
}

impl RaidUpdateEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            current_raid_wave: reader.var_i32(),
            total_raid_waves: reader.var_i32(),
            won_raid: reader.bool(),
        }
    }
}

impl EventData for RaidUpdateEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.current_raid_wave);
        writer.var_i32(self.total_raid_waves);
        writer.bool(self.won_raid);
    }

    fn event_type(&self) -> EventType {
        EventType::RaidUpdate
    }
}

#[derive(Debug)]
pub struct ReleaseItemTransactionData {
    pub action_type: u32,
    pub hot_bar_slot: i32,
    pub held_item: ItemInstance,
    pub head_position: Vec3,
}

impl ReleaseItemTransactionData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_u32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            head_position: reader.vec3(),
        }
    }
}

impl InventoryTransactionData for ReleaseItemTransactionData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.action_type);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.head_position);
    }

    fn transaction_type(&self) -> InventoryTransactionType {
        InventoryTransactionType::ReleaseItem
    }
}

#[derive(Debug)]
pub struct ScoreboardEntry {
    pub entry_id: i64,
    pub objective_name: String,
    pub score: i32,
    pub identity_type: ScoreboardIdentity,
    pub entity_unique_id: i64,
    pub display_name: String,
}

impl ScoreboardEntry {
    pub fn write(&self, writer: &mut Writer, action: ScoreboardAction) {
        writer.var_i64(self.entry_id);
        writer.string(self.objective_name.as_str());
        writer.i32(self.score);
        if action == ScoreboardAction::Modify {
            writer.u8(num::ToPrimitive::to_u8(&self.identity_type).unwrap());
            match self.identity_type {
                ScoreboardIdentity::Entity | ScoreboardIdentity::Player => {
                    writer.var_i64(self.entity_unique_id);
                }
                _ => {
                    writer.string(self.display_name.as_str());
                }
            }
        }
    }

    pub fn read(reader: &mut Reader, action: ScoreboardAction) -> Self {
        let mut entry = Self {
            entry_id: reader.var_i64(),
            objective_name: reader.string(),
            score: reader.i32(),
            identity_type: ScoreboardIdentity::Player,
            entity_unique_id: 0,
            display_name: "".into(),
        };
        if action == ScoreboardAction::Modify {
            entry.identity_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
            match entry.identity_type {
                ScoreboardIdentity::Entity | ScoreboardIdentity::Player => {
                    entry.entity_unique_id = reader.var_i64();
                }
                _ => {
                    entry.display_name = reader.string();
                }
            }
        }

        entry
    }
}

#[derive(Debug)]
pub struct ScoreboardIdentityEntry {
    pub entry_id: i64,
    pub entity_unique_id: i64,
}

impl ScoreboardIdentityEntry {
    pub fn write(&self, writer: &mut Writer, action: ScoreboardIdentityAction) {
        writer.var_i64(self.entry_id);
        if action == ScoreboardIdentityAction::Register {
            writer.var_i64(self.entity_unique_id);
        }
    }

    pub fn read(reader: &mut Reader, action: ScoreboardIdentityAction) -> Self {
        Self {
            entry_id: reader.var_i64(),
            entity_unique_id: if action == ScoreboardIdentityAction::Register { reader.var_i64() } else { 0 },
        }
    }
}

pub type ShapedChemistryRecipe = ShapedRecipe;

#[derive(Debug, Default)]
pub struct ShapedRecipe {
    pub recipe_id: String,
    pub width: i32,
    pub height: i32,
    pub input: Vec<ItemDescriptorCount>,
    pub output: Vec<ItemStack>,
    pub uuid: Uuid,
    pub block: String,
    pub priority: i32,
    pub recipe_network_id: u32,
}

impl ShapedRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.recipe_id.as_str());
        writer.i32(self.width);
        writer.i32(self.height);
        for i in 0..self.width * self.height {
            if i >= self.input.len() as i32 {
                ItemDescriptorCount::default().write(writer);
            } else {
                self.input[i as usize].write(writer);
            }
        }
        writer.var_u32(self.output.len() as u32);
        self.output.iter().for_each(|stack| stack.write(writer));
        writer.uuid(self.uuid);
        writer.string(self.block.as_str());
        writer.var_i32(self.priority);
        writer.var_u32(self.recipe_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        let recipe_id = reader.string();
        let width = reader.i32();
        let height = reader.i32();
        Self {
            recipe_id,
            width,
            height,
            input: (0..width * height).map(|_| ItemDescriptorCount::read(reader)).collect(),
            output: (0..reader.var_u32()).map(|_| ItemStack::read(reader)).collect(),
            uuid: reader.uuid(),
            block: reader.string(),
            priority: reader.var_i32(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

pub type ShapelessChemistryRecipe = ShapelessRecipe;

#[derive(Debug)]
pub struct ShapelessRecipe {
    pub recipe_id: String,
    pub input: Vec<ItemDescriptorCount>,
    pub output: Vec<ItemStack>,
    pub uuid: Uuid,
    pub block: String,
    pub priority: i32,
    pub recipe_network_id: u32,
}

impl ShapelessRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.recipe_id.as_str());
        writer.var_u32(self.input.len() as u32);
        self.input.iter().for_each(|input| input.write(writer));
        writer.var_u32(self.output.len() as u32);
        self.output.iter().for_each(|stack| stack.write(writer));
        writer.uuid(self.uuid);
        writer.string(self.block.as_str());
        writer.var_i32(self.priority);
        writer.var_u32(self.recipe_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_id: reader.string(),
            input: (0..reader.var_u32()).map(|_| ItemDescriptorCount::read(reader)).collect(),
            output: (0..reader.var_u32()).map(|_| ItemStack::read(reader)).collect(),
            uuid: reader.uuid(),
            block: reader.string(),
            priority: reader.var_i32(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

pub type ShulkerBoxRecipe = ShapelessRecipe;

#[derive(Debug, Default)]
pub struct Skin {
    pub skin_id: String,
    pub play_fab_id: String,
    pub skin_resource_patch: Bytes,
    pub skin_image_width: u32,
    pub skin_image_height: u32,
    pub skin_data: Bytes,
    pub animations: Vec<SkinAnimation>,
    pub cape_image_width: u32,
    pub cape_image_height: u32,
    pub cape_data: Bytes,
    pub skin_geometry: Bytes,
    pub geometry_data_engine_version: Bytes,
    pub animation_data: Bytes,
    pub cape_id: String,
    pub full_id: String,
    pub arm_size: String,
    pub skin_colour: String,
    pub persona_pieces: Vec<PersonaPiece>,
    pub piece_tint_colours: Vec<PersonaPieceTintColour>,
    pub premium_skin: bool,
    pub persona_skin: bool,
    pub persona_cape_on_classic_skin: bool,
    pub primary_user: bool,
    pub trusted: bool,
}

impl Skin {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.skin_id.as_str());
        writer.string(self.play_fab_id.as_str());
        writer.byte_slice(&self.skin_resource_patch);
        writer.u32(self.skin_image_width);
        writer.u32(self.skin_image_height);
        writer.byte_slice(&self.skin_data);
        writer.u32(self.animations.len() as u32);
        self.animations.iter().for_each(|animation| animation.write(writer));
        writer.u32(self.cape_image_width);
        writer.u32(self.cape_image_height);
        writer.byte_slice(&self.cape_data);
        writer.byte_slice(&self.skin_geometry);
        writer.byte_slice(&self.geometry_data_engine_version);
        writer.byte_slice(&self.animation_data);
        writer.string(self.cape_id.as_str());
        writer.string(self.full_id.as_str());
        writer.string(self.arm_size.as_str());
        writer.string(self.skin_colour.as_str());
        writer.u32(self.persona_pieces.len() as u32);
        self.persona_pieces.iter().for_each(|piece| piece.write(writer));
        writer.u32(self.piece_tint_colours.len() as u32);
        self.piece_tint_colours.iter().for_each(|colour| colour.write(writer));
        writer.bool(self.premium_skin);
        writer.bool(self.persona_skin);
        writer.bool(self.persona_cape_on_classic_skin);
        writer.bool(self.primary_user);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            skin_id: reader.string(),
            play_fab_id: reader.string(),
            skin_resource_patch: reader.byte_slice(),
            skin_image_width: reader.u32(),
            skin_image_height: reader.u32(),
            skin_data: reader.byte_slice(),
            animations: (0..reader.u32()).map(|_| SkinAnimation::read(reader)).collect(),
            cape_image_width: reader.u32(),
            cape_image_height: reader.u32(),
            cape_data: reader.byte_slice(),
            skin_geometry: reader.byte_slice(),
            geometry_data_engine_version: reader.byte_slice(),
            animation_data: reader.byte_slice(),
            cape_id: reader.string(),
            full_id: reader.string(),
            arm_size: reader.string(),
            skin_colour: reader.string(),
            persona_pieces: (0..reader.u32()).map(|_| PersonaPiece::read(reader)).collect(),
            piece_tint_colours: (0..reader.u32()).map(|_| PersonaPieceTintColour::read(reader)).collect(),
            premium_skin: reader.bool(),
            persona_skin: reader.bool(),
            persona_cape_on_classic_skin: reader.bool(),
            primary_user: reader.bool(),
            trusted: false,
        }
    }
}

#[derive(Debug)]
pub struct SkinAnimation {
    pub image_width: u32,
    pub image_height: u32,
    pub image_data: Bytes,
    pub animation_type: u32,
    pub frame_count: f32,
    pub expression_type: u32,
}

impl SkinAnimation {
    pub fn write(&self, writer: &mut Writer) {
        writer.u32(self.image_width);
        writer.u32(self.image_height);
        writer.byte_slice(&self.image_data);
        writer.u32(self.animation_type);
        writer.f32(self.frame_count);
        writer.u32(self.expression_type);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            image_width: reader.u32(),
            image_height: reader.u32(),
            image_data: reader.byte_slice(),
            animation_type: reader.u32(),
            frame_count: reader.f32(),
            expression_type: reader.u32(),
        }
    }
}

#[derive(Debug)]
pub struct SlashCommandExecutedEventData {
    pub success_count: i32,
    pub message_count: i32,
    pub command_name: String,
    pub output_messages: String,
}

impl SlashCommandExecutedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            success_count: reader.var_i32(),
            message_count: reader.var_i32(),
            command_name: reader.string(),
            output_messages: reader.string(),
        }
    }
}

impl EventData for SlashCommandExecutedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.success_count);
        writer.var_i32(self.message_count);
        writer.string(self.command_name.as_str());
        writer.string(self.output_messages.as_str());
    }

    fn event_type(&self) -> EventType {
        EventType::SlashCommandExecuted
    }
}

#[derive(Debug)]
pub struct SneakCloseToSculkSensorEventData {}

impl SneakCloseToSculkSensorEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

impl EventData for SneakCloseToSculkSensorEventData {
    fn write(&self, _: &mut Writer) {}

    fn event_type(&self) -> EventType {
        EventType::ExtractHoney
    }
}

pub trait StackRequestAction: Debug { // todo: remove
    fn write(&self, writer: &mut Writer);
    fn action_type(&self) -> StackRequestActionType;
}

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

impl Default for StackRequestAction2 {
    fn default() -> Self {
        Self::CraftNonImplementedStackRequestAction(CraftNonImplementedStackRequestAction{})
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StackResourcePack {
    pub uuid: String,
    pub version: String,
    pub sub_pack_name: String,
}

impl StackResourcePack {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.string(self.version.as_str());
        writer.string(self.sub_pack_name.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            version: reader.string(),
            sub_pack_name: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct StackResponseContainerInfo {
    pub container_id: u8,
    pub slot_info: Vec<StackResponseSlotInfo>,
}

impl StackResponseContainerInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.container_id);
        writer.var_u32(self.slot_info.len() as u32);
        self.slot_info.iter().for_each(|slot_info| slot_info.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            container_id: reader.u8(),
            slot_info: (0..reader.var_u32()).map(|_| StackResponseSlotInfo::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StructureSettings {
    pub palette_name: String,
    pub ignore_entities: bool,
    pub ignore_blocks: bool,
    pub allow_non_ticking_chunks: bool,
    pub size: BlockPos,
    pub offset: BlockPos,
    pub last_editing_player_unique_id: i64,
    pub rotation: u8,
    pub mirror: u8,
    pub animation_mode: u8,
    pub animation_duration: f32,
    pub integrity: f32,
    pub seed: u32,
    pub pivot: Vec3,
}

impl StructureSettings {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.palette_name.as_str());
        writer.bool(self.ignore_entities);
        writer.bool(self.ignore_blocks);
        writer.bool(self.allow_non_ticking_chunks);
        writer.u_block_pos(self.size);
        writer.u_block_pos(self.offset);
        writer.var_i64(self.last_editing_player_unique_id);
        writer.u8(self.rotation);
        writer.u8(self.mirror);
        writer.u8(self.animation_mode);
        writer.f32(self.animation_duration);
        writer.f32(self.integrity);
        writer.u32(self.seed);
        writer.vec3(self.pivot);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            palette_name: reader.string(),
            ignore_entities: reader.bool(),
            ignore_blocks: reader.bool(),
            allow_non_ticking_chunks: reader.bool(),
            size: reader.u_block_pos(),
            offset: reader.u_block_pos(),
            last_editing_player_unique_id: reader.var_i64(),
            rotation: reader.u8(),
            mirror: reader.u8(),
            animation_mode: reader.u8(),
            animation_duration: reader.f32(),
            integrity: reader.f32(),
            seed: reader.u32(),
            pivot: reader.vec3(),
        }
    }
}

#[derive(Debug)]
pub struct SubChunkEntry {
    pub offset: SubChunkOffset,
    pub result: SubChunkResult,
    pub raw_payload: Bytes,
    pub height_map_type: HeightMapType,
    pub height_map_data: [i8; 256],
    pub blob_hash: u64,
}

impl SubChunkEntry {
    pub fn write(&self, writer: &mut Writer, cache_enabled: bool) {
        self.offset.write(writer);
        writer.u8(num::ToPrimitive::to_u8(&self.result).unwrap());
        if self.result != SubChunkResult::SuccessAllAir || cache_enabled {
            writer.byte_slice(&self.raw_payload);
        }
        writer.u8(num::ToPrimitive::to_u8(&self.height_map_type).unwrap());
        if self.height_map_type == HeightMapType::HasData {
            for data in self.height_map_data {
                writer.i8(data);
            }
        }
        if !cache_enabled {
            writer.u64(self.blob_hash);
        }
    }

    pub fn read(reader: &mut Reader, cache_enabled: bool) -> Self {
        let mut entry = Self {
            offset: SubChunkOffset::read(reader),
            result: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            raw_payload: Bytes::default(),
            height_map_type: HeightMapType::None,
            height_map_data: [0; 256],
            blob_hash: 0,
        };
        if entry.result != SubChunkResult::SuccessAllAir || cache_enabled {
            entry.raw_payload = reader.byte_slice();
        }
        entry.height_map_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        if entry.height_map_type == HeightMapType::HasData {
            for i in 0..256 {
                entry.height_map_data[i] = reader.i8();
            }
        }
        if !cache_enabled {
            entry.blob_hash = reader.u64();
        }

        entry
    }
}

#[derive(Debug)]
pub struct SubChunkOffset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl SubChunkOffset {
    pub fn write(&self, writer: &mut Writer) {
        writer.i8(self.x);
        writer.i8(self.y);
        writer.i8(self.z);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            x: reader.i8(),
            y: reader.i8(),
            z: reader.i8(),
        }
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
pub struct TexturePackInfo {
    pub uuid: String,
    pub version: String,
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
    pub rtx_enabled: bool,
}

impl TexturePackInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.string(self.version.as_str());
        writer.u64(self.size);
        writer.string(self.content_key.as_str());
        writer.string(self.sub_pack_name.as_str());
        writer.string(self.content_identity.as_str());
        writer.bool(self.has_scripts);
        writer.bool(self.rtx_enabled);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            version: reader.string(),
            size: reader.u64(),
            content_key: reader.string(),
            sub_pack_name: reader.string(),
            content_identity: reader.string(),
            has_scripts: reader.bool(),
            rtx_enabled: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct UseItemOnEntityTransactionData {
    pub target_entity_runtime_id: u64,
    pub action_type: u32,
    pub hot_bar_slot: i32,
    pub held_item: ItemInstance,
    pub position: Vec3,
    pub clicked_position: Vec3,
}

impl UseItemOnEntityTransactionData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            target_entity_runtime_id: reader.var_u64(),
            action_type: reader.var_u32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            position: reader.vec3(),
            clicked_position: reader.vec3(),
        }
    }
}

impl InventoryTransactionData for UseItemOnEntityTransactionData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.target_entity_runtime_id);
        writer.var_u32(self.action_type);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.position);
        writer.vec3(self.clicked_position);
    }

    fn transaction_type(&self) -> InventoryTransactionType {
        InventoryTransactionType::UseItemOnEntity
    }
}

#[derive(Debug, Default)]
pub struct UseItemTransactionData {
    pub legacy_request_id: i32,
    pub legacy_set_item_slots: Vec<LegacySetItemSlot>,
    pub actions: Vec<InventoryAction>,
    pub action_type: u32,
    pub block_position: BlockPos,
    pub block_face: i32,
    pub hot_bar_slot: i32,
    pub held_item: ItemInstance,
    pub position: Vec3,
    pub clicked_position: Vec3,
    pub block_runtime_id: u32,
}

impl UseItemTransactionData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_u32(),
            block_position: reader.u_block_pos(),
            block_face: reader.var_i32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            position: reader.vec3(),
            clicked_position: reader.vec3(),
            block_runtime_id: reader.var_u32(),
            ..Default::default()
        }
    }

    pub fn write_player_action(&self, writer: &mut Writer) {
        writer.var_i32(self.legacy_request_id);
        if self.legacy_request_id < -1 && (self.legacy_request_id & 1) == 0 {
            writer.var_u32(self.legacy_set_item_slots.len() as u32);
            self.legacy_set_item_slots.iter().for_each(|slot| slot.write(writer));
        }
        writer.var_u32(self.actions.len() as u32);
        self.actions.iter().for_each(|action| action.write(writer));
        writer.var_u32(self.action_type);
        writer.block_pos(self.block_position);
        writer.var_i32(self.block_face);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.position);
        writer.vec3(self.clicked_position);
        writer.var_u32(self.block_runtime_id);
    }

    pub fn read_player_action(reader: &mut Reader) -> Self {
        let legacy_request_id = reader.var_i32();
        Self {
            legacy_request_id,
            legacy_set_item_slots: if legacy_request_id < -1 && (legacy_request_id & 1) == 0 {
                (0..reader.var_u32()).map(|_| LegacySetItemSlot::read(reader)).collect()
            } else {
                Vec::new()
            },
            actions: (0..reader.var_u32()).map(|_| InventoryAction::read(reader)).collect(),
            action_type: reader.var_u32(),
            block_position: reader.block_pos(),
            block_face: reader.var_i32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            position: reader.vec3(),
            clicked_position: reader.vec3(),
            block_runtime_id: reader.var_u32(),
        }
    }
}

impl InventoryTransactionData for UseItemTransactionData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.action_type);
        writer.u_block_pos(self.block_position);
        writer.var_i32(self.block_face);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.position);
        writer.vec3(self.clicked_position);
        writer.var_u32(self.block_runtime_id);
    }

    fn transaction_type(&self) -> InventoryTransactionType {
        InventoryTransactionType::UseItem
    }
}
