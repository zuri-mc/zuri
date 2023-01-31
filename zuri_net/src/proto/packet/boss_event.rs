use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BossEventType {
    Show,
    RegisterPlayer,
    Hide,
    UnregisterPlayer,
    HealthPercentage,
    Title,
    AppearanceProperties,
    Texture,
    Request,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum BossEventColour {
    Grey,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

/// Sent by the server to make a specific 'boss event' occur in the world. It includes features such
/// as showing a boss bar to the player and turning the sky dark.
#[derive(Debug, Clone)]
pub struct BossEvent {
    /// The unique ID of the boss entity that the boss event sent involves. The health percentage
    /// and title of the boss bar depend on the health and name tag of this entity.
    pub boss_entity_unique_id: i64,
    /// The type of the event. The fields written depend on the event type set, and some event types
    /// are sent by the client, whereas others are sent by the server.
    pub event_type: BossEventType,
    /// The unique ID of the player that is registered to or unregistered from the boss fight. It is
    /// set if the event type is either register player or unregister player.
    pub player_unique_id: i64,
    /// The title shown above the boss bar. It currently does not function, and instead uses the
    /// name-tag of the boss entity at all times. It is only set if the event type is show or title.
    pub boss_bar_title: String,
    /// The percentage of health that is shown in the boss bar. It currently does not function, and
    /// instead uses the health percentage of the boss entity at all times. It is only set if the
    /// event type is show or health percentage.
    pub health_percentage: f32,
    /// The purpose of this field is currently unknown.
    pub screen_darkening: i16,
    /// The colour of the boss bar that is shown when a player is subscribed. It is only set if the
    /// event type is show, appearance properties or texture. This is functional as of 1.18.
    pub colour: BossEventColour,
    /// The overlay of the boss bar that is shown on top of the boss bar when a player is
    /// subscribed. It currently does not function. It is only set if the event type is show,
    /// appearance properties or texture.
    pub overlay: u32,
}

impl PacketType for BossEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.boss_entity_unique_id);
        writer.var_u32(self.event_type.to_u32().unwrap());
        match self.event_type {
            BossEventType::Show => {
                writer.string(self.boss_bar_title.as_str());
                writer.f32(self.health_percentage);
                writer.i16(self.screen_darkening);
                writer.var_u32(self.colour.to_u32().unwrap());
                writer.var_u32(self.overlay);
            }
            BossEventType::RegisterPlayer | BossEventType::UnregisterPlayer | BossEventType::Request => {
                writer.var_i64(self.player_unique_id);
            }
            BossEventType::Hide => {}
            BossEventType::HealthPercentage => {
                writer.f32(self.health_percentage);
            }
            BossEventType::Title => {
                writer.string(self.boss_bar_title.as_str());
            }
            BossEventType::AppearanceProperties => {
                writer.i16(self.screen_darkening.to_i16().unwrap());
                writer.var_u32(self.colour.to_u32().unwrap());
                writer.var_u32(self.overlay);
            }
            BossEventType::Texture => {
                writer.var_u32(self.colour.to_u32().unwrap());
                writer.var_u32(self.overlay);
            }
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let boss_entity_unique_id = reader.var_i64();
        let event_type = BossEventType::from_u32(reader.var_u32()).unwrap();
        Self {
            boss_entity_unique_id,
            event_type,
            player_unique_id: if event_type == BossEventType::RegisterPlayer || event_type == BossEventType::UnregisterPlayer || event_type == BossEventType::Request {
                reader.var_i64()
            } else {
                0
            },
            boss_bar_title: if event_type == BossEventType::Show || event_type == BossEventType::Title { reader.string() } else { String::new() },
            health_percentage: if event_type == BossEventType::Show || event_type == BossEventType::HealthPercentage { reader.f32() } else { 0.0 },
            screen_darkening: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties { reader.i16() } else { 0 },
            colour: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties || event_type == BossEventType::Texture {
                BossEventColour::from_u32(reader.var_u32()).unwrap()
            } else {
                BossEventColour::Purple
            },
            overlay: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties || event_type == BossEventType::Texture {
                reader.var_u32()
            } else {
                0
            },
        }
    }
}
