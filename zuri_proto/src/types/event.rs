use crate::encodable_enum;
use crate::io::{Reader, Writer};

encodable_enum!(
    #[derive(Debug)]
    pub enum EventType {
        AchievementAwarded = 0,
        EntityInteract = 1,
        PortalBuilt = 2,
        PortalUsed = 3,
        MobKilled = 4,
        CauldronUsed = 5,
        PlayerDied = 6,
        BossKilled = 7,
        AgentCommand = 8,
        AgentCreated = 9,
        PatternRemoved = 10,
        SlashCommandExecuted = 11,
        FishBucketed = 12,
        MobBorn = 13,
        PetDied = 14,
        CauldronInteract = 15,
        ComposterInteract = 16,
        BellUsed = 17,
        EntityDefinitionTrigger = 18,
        //RaidUpdate = 19, TODO
        MovementAnomaly = 20,
        MovementCorrected = 21,
        //ExtractHoney = 22, todo
        //TargetBlockHit = 23, todo
        //PiglinBarter = 24, todo
        PlayerWaxedOrUnwaxedCopper = 25,
        //CodeBuilderRuntimeAction = 26, todo
        //CodeBuilderScoreboard = 27, todo
        //StriderRiddenInLavaInOverworld = 28, todo
        SneakCloseToSculkSensor = 29,
    }
);

#[derive(Debug)]
pub struct EntityDefinitionTrigger {
    pub event_name: String,
}

impl EntityDefinitionTrigger{
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            event_name: reader.string(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.event_name.as_str());
    }
}

#[derive(Debug)]
pub struct EntityInteract {
    pub interaction_type: i32,
    pub interaction_entity_type: i32,
    pub entity_variant: i32,
    pub entity_colour: u8,
}

impl EntityInteract {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            interaction_type: reader.var_i32(),
            interaction_entity_type: reader.var_i32(),
            entity_variant: reader.var_i32(),
            entity_colour: reader.u8(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.interaction_type);
        writer.var_i32(self.interaction_entity_type);
        writer.var_i32(self.entity_variant);
        writer.u8(self.entity_colour);
    }
}

#[derive(Debug)]
pub struct CauldronInteract {
    pub block_interaction_type: i32,
    pub item_id: i32,
}

impl CauldronInteract {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_interaction_type: reader.var_i32(),
            item_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.block_interaction_type);
        writer.var_i32(self.item_id);
    }
}

#[derive(Debug)]
pub struct CauldronUsed {
    pub potion_id: i32,
    pub colour: i32,
    pub fill_level: i32,
}

impl CauldronUsed {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            potion_id: reader.var_i32(),
            colour: reader.var_i32(),
            fill_level: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.potion_id);
        writer.var_i32(self.colour);
        writer.var_i32(self.fill_level);
    }
}

#[derive(Debug)]
pub struct ComposterInteract {
    pub block_interaction_type: i32,
    pub item_id: i32,
}

impl ComposterInteract {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            block_interaction_type: reader.var_i32(),
            item_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.block_interaction_type);
        writer.var_i32(self.item_id);
    }
}

#[derive(Debug)]
pub struct BossKilled {
    pub boss_entity_unique_id: i64,
    pub player_party_size: i32,
    pub interaction_entity_type: i32,
}

impl BossKilled {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            boss_entity_unique_id: reader.var_i64(),
            player_party_size: reader.var_i32(),
            interaction_entity_type: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.boss_entity_unique_id);
        writer.var_i32(self.player_party_size);
        writer.var_i32(self.interaction_entity_type);
    }
}

#[derive(Debug)]
pub struct AchievementAwarded {
    pub achievement_id: i32,
}

impl AchievementAwarded {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            achievement_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.achievement_id);
    }
}

#[derive(Debug)]
pub struct AgentCommand {
    pub agent_result: i32,
    pub data_value: i32,
    pub command: String,
    pub data_key: String,
    pub output: String,
}

impl AgentCommand {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            agent_result: reader.var_i32(),
            data_value: reader.var_i32(),
            command: reader.string(),
            data_key: reader.string(),
            output: reader.string(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.agent_result);
        writer.var_i32(self.data_value);
        writer.string(self.command.as_str());
        writer.string(self.data_key.as_str());
        writer.string(self.output.as_str());
    }
}

#[derive(Debug)]
pub struct AgentCreated {}

impl AgentCreated {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug)]
pub struct SlashCommandExecuted {
    pub success_count: i32,
    pub message_count: i32,
    pub command_name: String,
    pub output_messages: String,
}

impl SlashCommandExecuted {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            success_count: reader.var_i32(),
            message_count: reader.var_i32(),
            command_name: reader.string(),
            output_messages: reader.string(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.success_count);
        writer.var_i32(self.message_count);
        writer.string(self.command_name.as_str());
        writer.string(self.output_messages.as_str());
    }
}

#[derive(Debug)]
pub struct MobKilled {
    pub killer_entity_unique_id: i64,
    pub victim_entity_unique_id: i64,
    pub killer_entity_type: i32,
    pub entity_damage_cause: i32,
    pub villager_trade_tier: i32,
    pub villager_display_name: String,
}

impl MobKilled {
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

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.killer_entity_unique_id);
        writer.var_i64(self.victim_entity_unique_id);
        writer.var_i32(self.killer_entity_type);
        writer.var_i32(self.entity_damage_cause);
        writer.var_i32(self.villager_trade_tier);
        writer.string(self.villager_display_name.as_str());
    }
}

#[derive(Debug)]
pub struct MovementAnomaly {
    pub event_type: u8,
    pub cheating_score: f32,
    pub average_position_delta: f32,
    pub total_position_delta: f32,
    pub min_position_delta: f32,
    pub max_position_delta: f32,
}

impl MovementAnomaly {
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

    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.event_type);
        writer.f32(self.cheating_score);
        writer.f32(self.average_position_delta);
        writer.f32(self.total_position_delta);
        writer.f32(self.min_position_delta);
        writer.f32(self.max_position_delta);
    }
}

#[derive(Debug)]
pub struct MovementCorrected {
    pub position_delta: f32,
    pub cheating_score: f32,
    pub score_threshold: f32,
    pub distance_threshold: f32,
    pub duration_threshold: i32,
}

impl MovementCorrected {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            position_delta: reader.f32(),
            cheating_score: reader.f32(),
            score_threshold: reader.f32(),
            distance_threshold: reader.f32(),
            duration_threshold: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.f32(self.position_delta);
        writer.f32(self.cheating_score);
        writer.f32(self.score_threshold);
        writer.f32(self.distance_threshold);
        writer.var_i32(self.duration_threshold);
    }
}

#[derive(Debug)]
pub struct BellUsed {
    pub item_id: i32,
}

impl BellUsed {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.item_id);
    }
}

#[derive(Debug)]
pub struct ExtractHoney {} // todo

impl ExtractHoney {
    pub fn read(_reader: &mut Reader) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct FishBucketed {
    pub pattern: i32,
    pub preset: i32,
    pub bucketed_entity_type: i32,
    pub release: bool,
}

impl FishBucketed {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            pattern: reader.var_i32(),
            preset: reader.var_i32(),
            bucketed_entity_type: reader.var_i32(),
            release: reader.bool(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.pattern);
        writer.var_i32(self.preset);
        writer.var_i32(self.bucketed_entity_type);
        writer.bool(self.release);
    }
}

#[derive(Debug)]
pub struct MobBorn {
    pub entity_type: i32,
    pub variant: i32,
    pub colour: u8,
}

impl MobBorn {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            entity_type: reader.var_i32(),
            variant: reader.var_i32(),
            colour: reader.u8(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
        writer.var_i32(self.variant);
        writer.u8(self.colour);
    }
}

#[derive(Debug)]
pub struct PlayerWaxedOrUnwaxedCopper {}

impl PlayerWaxedOrUnwaxedCopper {
    pub fn read(_reader: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug)]
pub struct PetDied {
    pub killed_by_owner: bool,
    pub killer_entity_unique_id: i64,
    pub pet_entity_unique_id: i64,
    pub entity_damage_cause: i32,
    pub pet_entity_type: i32,
}

impl PetDied {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            killed_by_owner: reader.bool(),
            killer_entity_unique_id: reader.var_i64(),
            pet_entity_unique_id: reader.var_i64(),
            entity_damage_cause: reader.var_i32(),
            pet_entity_type: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.bool(self.killed_by_owner);
        writer.var_i64(self.killer_entity_unique_id);
        writer.var_i64(self.pet_entity_unique_id);
        writer.var_i32(self.entity_damage_cause);
        writer.var_i32(self.pet_entity_type);
    }
}

#[derive(Debug)]
pub struct PlayerDied {
    pub attacker_entity_id: i32,
    pub attacker_variant: i32,
    pub entity_damage_cause: i32,
    pub in_raid: bool,
}

impl PlayerDied {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            attacker_entity_id: reader.var_i32(),
            attacker_variant: reader.var_i32(),
            entity_damage_cause: reader.var_i32(),
            in_raid: reader.bool(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.attacker_entity_id);
        writer.var_i32(self.attacker_variant);
        writer.var_i32(self.entity_damage_cause);
        writer.bool(self.in_raid);
    }
}

#[derive(Debug)]
pub struct PortalBuilt {
    pub dimension_id: i32,
}

impl PortalBuilt {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            dimension_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension_id);
    }
}

#[derive(Debug)]
pub struct PortalUsed {
    pub from_dimension_id: i32,
    pub to_dimension_id: i32,
}

impl PortalUsed {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            from_dimension_id: reader.var_i32(),
            to_dimension_id: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.from_dimension_id);
        writer.var_i32(self.to_dimension_id);
    }
}

#[derive(Debug)]
pub struct SneakCloseToSculkSensor {}

impl SneakCloseToSculkSensor {
    pub fn read(_reader: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug)]
pub struct PatternRemoved {
    pub item_id: i32,
    pub aux_value: i32,
    pub patterns_size: i32,
    pub pattern_index: i32,
    pub pattern_colour: i32,
}

impl PatternRemoved {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_id: reader.var_i32(),
            aux_value: reader.var_i32(),
            patterns_size: reader.var_i32(),
            pattern_index: reader.var_i32(),
            pattern_colour: reader.var_i32(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.item_id);
        writer.var_i32(self.aux_value);
        writer.var_i32(self.patterns_size);
        writer.var_i32(self.pattern_index);
        writer.var_i32(self.pattern_colour);
    }
}