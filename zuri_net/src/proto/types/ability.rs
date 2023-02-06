use num_derive::{FromPrimitive, ToPrimitive};
use zuri_net_derive::packet;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Ability {
    Build,
    Mine,
    DoorsAndSwitches,
    OpenContainers,
    AttackPlayers,
    AttackMobs,
    OperatorCommands,
    Teleport,
    Invulnerable,
    Flying,
    MayFly,
    InstantBuild,
    Lightning,
    FlySpeed,
    WalkSpeed,
    Muted,
    WorldBuilder,
    NoClip,
    Count,
}

#[packet(u16)]
#[derive(Debug, Clone)]
pub enum AbilityLayerType {
    CustomCache,
    Base,
    Spectator,
    Commands,
    Editor,
}

#[packet]
#[derive(Debug, Clone)]
pub struct AbilityData {
    pub entity_unique_id: i64,
    pub player_permissions: u8,
    pub command_permission: u8,
    #[size_type(u8)]
    pub layers: Vec<AbilityLayer>,
}

#[packet]
#[derive(Debug, Clone)]
pub struct AbilityLayer {
    layer_type: AbilityLayerType,
    abilities: u32,
    values: u32,
    fly_speed: f32,
    walk_speed: f32,
}
