use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum BossEventColour {
    Grey,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug)]
pub struct BossEvent {
    pub boss_entity_unique_id: i64,
    pub event_type: BossEventType,
    pub player_unique_id: i64,
    pub boss_bar_title: String,
    pub health_percentage: f32,
    pub screen_darkening: i16,
    pub colour: BossEventColour,
    pub overlay: u32,
}

impl PacketType for BossEvent {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.boss_entity_unique_id);
        writer.u32(self.event_type.to_u32().unwrap());
        match self.event_type {
            BossEventType::Show => {
                writer.string(self.boss_bar_title.as_str());
                writer.f32(self.health_percentage);
                writer.i16(self.screen_darkening);
                writer.u32(self.colour.to_u32().unwrap());
                writer.u32(self.overlay);
            }
            BossEventType::RegisterPlayer | BossEventType::UnregisterPlayer | BossEventType::Request => {
                writer.i64(self.player_unique_id);
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
                writer.u32(self.colour.to_u32().unwrap());
                writer.u32(self.overlay);
            }
            BossEventType::Texture => {
                writer.u32(self.colour.to_u32().unwrap());
                writer.u32(self.overlay);
            }
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let boss_entity_unique_id = reader.i64();
        let event_type = BossEventType::from_u32(reader.u32()).unwrap();
        Self {
            boss_entity_unique_id,
            event_type,
            player_unique_id: if event_type == BossEventType::RegisterPlayer || event_type == BossEventType::UnregisterPlayer || event_type == BossEventType::Request {
                reader.i64()
            } else {
                0
            },
            boss_bar_title: if event_type == BossEventType::Show || event_type == BossEventType::Title { reader.string() } else { "".to_string() },
            health_percentage: if event_type == BossEventType::Show || event_type == BossEventType::HealthPercentage { reader.f32() } else { 0.0 },
            screen_darkening: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties { reader.i16() } else { 0 },
            colour: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties || event_type == BossEventType::Texture {
                BossEventColour::from_u32(reader.u32()).unwrap()
            } else {
                BossEventColour::Purple
            },
            overlay: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties || event_type == BossEventType::Texture {
                reader.u32()
            } else {
                0
            },
        }
    }
}
