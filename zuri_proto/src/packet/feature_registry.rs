#[derive(Debug)]
pub struct FeatureRegistry {
    pub features: Vec<GenerationFeature>,
}

impl Packet for FeatureRegistry {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.features.len() as u32);
        self.features.iter().for_each(|f| f.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { features: (0..reader.var_u32()).map(|_| GenerationFeature::read(reader)).collect() }
    }
}
