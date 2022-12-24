#[derive(Debug)]
pub struct LevelSoundEvent {
    pub sound_type: u32,
    pub position: Vec3,
    pub extra_data: i32,
    pub entity_type: String,
    pub baby_mob: bool,
    pub disable_relative_volume: bool,
}

impl Packet for LevelSoundEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.sound_type);
        writer.vec3(self.position);
        writer.var_i32(self.extra_data);
        writer.string(self.entity_type.as_str());
        writer.bool(self.baby_mob);
        writer.bool(self.disable_relative_volume);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_type: reader.var_u32(),
            position: reader.vec3(),
            extra_data: reader.var_i32(),
            entity_type: reader.string(),
            baby_mob: reader.bool(),
            disable_relative_volume: reader.bool(),
        }
    }
}
