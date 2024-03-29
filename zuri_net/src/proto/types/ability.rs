use num_derive::{FromPrimitive, ToPrimitive};
use zuri_net_derive::proto;

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
    PrivilegedBuilder,
    Count,
}

#[proto(u16)]
#[derive(Debug, Clone)]
pub enum AbilityLayerType {
    CustomCache,
    Base,
    Spectator,
    Commands,
    Editor,
}

#[proto]
#[derive(Debug, Clone)]
pub struct AbilityData {
    pub entity_unique_id: i64,
    pub player_permissions: u8,
    pub command_permission: u8,
    #[len_type(u8)]
    pub layers: Vec<AbilityLayer>,
}

#[proto]
#[derive(Debug, Clone)]
pub struct AbilityLayer {
    layer_type: AbilityLayerType,
    abilities: u32,
    values: u32,
    fly_speed: f32,
    walk_speed: f32,
}
