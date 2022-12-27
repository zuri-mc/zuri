use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::GenerationFeature;

#[derive(Debug)]
pub struct FeatureRegistry {
    pub features: Vec<GenerationFeature>,
}

impl PacketType for FeatureRegistry {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.features.len() as u32);
        self.features.iter().for_each(|f| f.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { features: (0..reader.var_u32()).map(|_| GenerationFeature::read(reader)).collect() }
    }
}
