use crate::encodable_enum;
use crate::io::{Reader, Writer};

encodable_enum!(
    #[derive(Debug)]
    pub enum EventType {
        AchievementAwardedEventType = 0,
        EntityInteractEventType = 1,
        PortalBuiltEventType = 2,
        PortalUsedEventType = 3,
        MobKilledEventType = 4,
        CauldronUsedEventType = 5,
        PlayerDiedEventType = 6,
        BossKilledEventType = 7,
        AgentCommandEventType = 8,
        AgentCreatedEventType = 9,
        PatternRemovedEventType = 10,
        SlashCommandExecutedEventType = 11,
        FishBucketedEventType = 12,
        MobBornEventType = 13,
        PetDiedEventType = 14,
        CauldronInteractEventType = 15,
        ComposterInteractEventType = 16,
        BellUsedEventType = 17,
        EntityDefinitionTriggerEventType = 18,
        RaidUpdateEventType = 19,
        MovementAnomalyEventType = 20,
        MovementCorrectedEventType = 21,
        //ExtractHoneyEventType = 22, todo
        //TargetBlockHitEventType = 23, todo
        //PiglinBarterEventType = 24, todo
        PlayerWaxedOrUnwaxedCopperEventType = 25,
        //CodeBuilderRuntimeActionEventType = 26, todo
        //CodeBuilderScoreboardEventType = 27, todo
        //StriderRiddenInLavaInOverworldEventType = 28, todo
        SneakCloseToSculkSensorEventType = 29,
    }
);

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
pub struct ExtractHoneyEventData {} // todo

impl ExtractHoneyEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

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
