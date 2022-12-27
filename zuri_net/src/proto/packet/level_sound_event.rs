use glam::Vec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::sound_event::SoundEvent;

#[derive(Debug)]
pub struct LevelSoundEvent {
    pub sound: SoundEvent,
    pub position: Vec3,
    pub extra_data: i32,
    pub entity_type: String,
    pub baby_mob: bool,
    pub disable_relative_volume: bool,
}

impl PacketType for LevelSoundEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.sound.to_u32().unwrap());
        writer.vec3(self.position);
        writer.var_i32(self.extra_data);
        writer.string(self.entity_type.as_str());
        writer.bool(self.baby_mob);
        writer.bool(self.disable_relative_volume);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound: SoundEvent::from_u32(reader.var_u32()).unwrap(),
            position: reader.vec3(),
            extra_data: reader.var_i32(),
            entity_type: reader.string(),
            baby_mob: reader.bool(),
            disable_relative_volume: reader.bool(),
        }
    }
}
