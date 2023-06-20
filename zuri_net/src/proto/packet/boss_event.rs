use crate::proto::ints::{VarI64, VarU32};
use zuri_net_derive::proto;

/// Sent by the server to make a specific 'boss event' occur in the world. It includes features such
/// as showing a boss bar to the player and turning the sky dark.
#[proto]
#[derive(Debug, Clone)]
pub struct BossEvent {
    /// The unique ID of the boss entity that the boss event sent involves. The health percentage
    /// and title of the boss bar depend on the health and name tag of this entity.
    pub boss_entity_unique_id: VarI64,
    /// The type of the event. Some event types
    /// are sent by the client, whereas others are sent by the server.
    pub event_type: BossEventType,
}

#[proto(VarU32)]
#[derive(Debug, Clone, PartialEq)]
pub enum BossEventColour {
    Grey,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[proto(VarU32)]
#[derive(Debug, Clone, PartialEq)]
pub enum BossEventType {
    Show(BossEventShow),
    RegisterPlayer(BossEventRegisterPlayer),
    Hide,
    UnregisterPlayer(BossEventUnregisterPlayer),
    HealthPercentage(BossEventHealthPercentage),
    Title(BossEventTitle),
    AppearanceProperties(BossEventAppearanceProperties),
    Texture(BossEventTexture),
    Request(BossEventRequest),
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventShow {
    /// The title shown above the boss bar. It may be set to a different title if the
    /// BossEntityUniqueID matches the client's entity unique ID.
    pub boss_bar_title: String,
    /// The percentage of health that is shown in the boss bar. The HealthPercentage may be set to a
    /// specific value if the BossEntityUniqueID matches the client's entity unique ID
    pub health_percentage: f32,
    /// The purpose of this field is currently unknown.
    pub screen_darkening: i16,
    /// The colour of the boss bar that is shown when a player is subscribed. This is functional as
    /// of 1.18.
    pub colour: BossEventColour,
    /// The overlay of the boss bar that is shown on top of the boss bar when a player is
    /// subscribed. It currently does not function.
    pub overlay: VarU32,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventRegisterPlayer {
    /// The unique ID of the player that is registered to or unregistered from the boss fight.
    pub player_unique_id: VarI64,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventUnregisterPlayer {
    /// The unique ID of the player that is registered to or unregistered from the boss fight.
    pub player_unique_id: VarI64,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventRequest {
    /// The unique ID of the player that is registered to or unregistered from the boss fight.
    pub player_unique_id: VarI64,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventHealthPercentage {
    /// The percentage of health that is shown in the boss bar. It currently does not function, and
    /// instead uses the health percentage of the boss entity at all times.
    pub health_percentage: f32,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventTitle {
    /// The title shown above the boss bar. It currently does not function, and instead uses the
    /// name-tag of the boss entity at all times.
    pub boss_bar_title: String,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventAppearanceProperties {
    /// The purpose of this field is currently unknown.
    pub screen_darkening: i16,
    /// The colour of the boss bar that is shown when a player is subscribed. This is functional as
    /// of 1.18.
    pub colour: BossEventColour,
    /// The overlay of the boss bar that is shown on top of the boss bar when a player is
    /// subscribed. It currently does not function.
    pub overlay: VarU32,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct BossEventTexture {
    /// The colour of the boss bar that is shown when a player is subscribed. This is functional as
    /// of 1.18.
    pub colour: BossEventColour,
    /// The overlay of the boss bar that is shown on top of the boss bar when a player is
    /// subscribed. It currently does not function.
    pub overlay: VarU32,
}
