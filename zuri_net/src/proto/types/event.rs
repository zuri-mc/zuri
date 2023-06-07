use crate::proto::ints::{VarI32, VarI64};
use zuri_net_derive::proto;

#[proto(VarI32)]
#[repr(i32)]
#[derive(Debug, Clone)]
pub enum EventType {
    AchievementAwarded(AchievementAwarded),
    EntityInteract(EntityInteract),
    PortalBuilt(PortalBuilt),
    PortalUsed(PortalUsed),
    MobKilled(MobKilled),
    CauldronUsed(CauldronUsed),
    PlayerDied(PlayerDied),
    BossKilled(BossKilled),
    AgentCommand(AgentCommand),
    AgentCreated(AgentCreated),
    PatternRemoved(PatternRemoved),
    SlashCommandExecuted(SlashCommandExecuted),
    FishBucketed(FishBucketed),
    MobBorn(MobBorn),
    PetDied(PetDied),
    CauldronInteract(CauldronInteract),
    ComposterInteract(ComposterInteract),
    BellUsed(BellUsed),
    EntityDefinitionTrigger(EntityDefinitionTrigger),
    RaidUpdate(RaidUpdate),
    MovementAnomaly(MovementAnomaly),
    MovementCorrected(MovementCorrected),
    ExtractHoney(ExtractHoney),
    TargetBlockHit(TargetBlockHit),
    PiglinBarter(PiglinBarter),
    PlayerWaxedOrUnwaxedCopper(PlayerWaxedOrUnwaxedCopper),
    CodeBuilderRuntimeAction(CodeBuilderRuntimeAction),
    CodeBuilderScoreboard(CodeBuilderScoreboard),
    StriderRiddenInLavaInOverworld(StriderRiddenInLavaInOverworld),
    SneakCloseToSculkSensor(SneakCloseToSculkSensor),
    CarefulRestoration(CarefulRestoration),
}

#[proto]
#[derive(Debug, Clone)]
pub struct StriderRiddenInLavaInOverworld {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[proto]
#[derive(Debug, Clone)]
pub struct CodeBuilderScoreboard {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[proto]
#[derive(Debug, Clone)]
pub struct CodeBuilderRuntimeAction {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[proto]
#[derive(Debug, Clone)]
pub struct PiglinBarter {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

#[proto]
#[derive(Debug, Clone)]
pub struct TargetBlockHit {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    // The structure of this event is unknown.
}

/// The purpose of this event is unknown.
#[proto]
#[derive(Debug, Clone)]
pub struct ExtractHoney {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

/// Used to update a raids progress client side.
#[proto]
#[derive(Debug, Clone)]
pub struct RaidUpdate {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub current_raid_wave: VarI32,
    pub total_raid_waves: VarI32,
    pub won_raid: bool,
}

/// The purpose of this event is unknown.
#[proto]
#[derive(Debug, Clone)]
pub struct EntityDefinitionTrigger {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub event_name: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct EntityInteract {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub interaction_type: VarI32,
    pub interaction_entity_type: VarI32,
    pub entity_variant: VarI32,
    pub entity_colour: u8,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CauldronInteract {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub block_interaction_type: VarI32,
    pub item_id: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CauldronUsed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub potion_id: VarI32,
    pub colour: VarI32,
    pub fill_level: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct ComposterInteract {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub block_interaction_type: VarI32,
    pub item_id: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct BossKilled {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub boss_entity_unique_id: VarI64,
    pub player_party_size: VarI32,
    pub interaction_entity_type: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct AchievementAwarded {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub achievement_id: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct AgentCommand {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub agent_result: VarI32,
    pub data_value: VarI32,
    pub command: String,
    pub data_key: String,
    pub output: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct AgentCreated {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

#[proto]
#[derive(Debug, Clone)]
pub struct SlashCommandExecuted {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub success_count: VarI32,
    pub message_count: VarI32,
    pub command_name: String,
    pub output_messages: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct MobKilled {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub killer_entity_unique_id: VarI64,
    pub victim_entity_unique_id: VarI64,
    pub killer_entity_type: VarI32,
    pub entity_damage_cause: VarI32,
    pub villager_trade_tier: VarI32,
    pub villager_display_name: String,
}

/// Informs the receiver on movement data.
#[proto]
#[derive(Debug, Clone)]
pub struct MovementAnomaly {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub event_type: u8,
    pub cheating_score: f32,
    pub average_position_delta: f32,
    pub total_position_delta: f32,
    pub min_position_delta: f32,
    pub max_position_delta: f32,
}

/// Sent by the server to correct client-side movement.
#[proto]
#[derive(Debug, Clone)]
pub struct MovementCorrected {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub position_delta: f32,
    pub cheating_score: f32,
    pub score_threshold: f32,
    pub distance_threshold: f32,
    pub duration_threshold: VarI32,
}

/// This event is self-explanatory.
#[proto]
#[derive(Debug, Clone)]
pub struct BellUsed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub item_id: VarI32,
}

/// Sent when a fish is bucketed.
#[proto]
#[derive(Debug, Clone)]
pub struct FishBucketed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub pattern: VarI32,
    pub preset: VarI32,
    pub bucketed_entity_type: VarI32,
    pub release: bool,
}

/// Sent when a mob is born.
#[proto]
#[derive(Debug, Clone)]
pub struct MobBorn {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub entity_type: VarI32,
    pub variant: VarI32,
    pub colour: u8,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PlayerWaxedOrUnwaxedCopper {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

/// Sent when a pet dies. This event is deprecated.
#[proto]
#[derive(Debug, Clone)]
pub struct PetDied {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub killed_by_owner: bool,
    pub killer_entity_unique_id: VarI64,
    pub pet_entity_unique_id: VarI64,
    pub entity_damage_cause: VarI32,
    pub pet_entity_type: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PlayerDied {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub attacker_entity_id: VarI32,
    pub attacker_variant: VarI32,
    pub entity_damage_cause: VarI32,
    pub in_raid: bool,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PortalBuilt {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub dimension_id: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PortalUsed {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub from_dimension_id: VarI32,
    pub to_dimension_id: VarI32,
}

/// This event is self-explanatory.
#[proto]
#[derive(Debug, Clone)]
pub struct SneakCloseToSculkSensor {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

#[proto]
#[derive(Debug, Clone)]
pub struct CarefulRestoration {
    /// It is unclear what this field does.
    pub use_player_id: u8,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PatternRemoved {
    /// It is unclear what this field does.
    pub use_player_id: u8,
    pub item_id: VarI32,
    pub aux_value: VarI32,
    pub patterns_size: VarI32,
    pub pattern_index: VarI32,
    pub pattern_colour: VarI32,
}
