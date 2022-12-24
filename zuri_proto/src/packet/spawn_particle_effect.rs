use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct SpawnParticleEffect {
    pub dimension: u8,
    pub entity_unique_id: i64,
    pub position: Vec3,
    pub particle_name: String,
    pub molang_variables: Option<Bytes>,
}

impl Packet for SpawnParticleEffect {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.dimension);
        writer.var_i64(self.entity_unique_id);
        writer.vec3(self.position);
        writer.string(self.particle_name.as_str());
        writer.optional(&self.molang_variables);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: reader.u8(),
            entity_unique_id: reader.var_i64(),
            position: reader.vec3(),
            particle_name: reader.string(),
            molang_variables: reader.optional(),
        }
    }
}
