use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::GenerationFeature;

/// Notifies the client about the world generation features the server is currently using. This is
/// used in combination with the client-side world generation system introduced in v1.19.20,
/// allowing the client to completely generate the chunks of the world without having to rely on the
/// server.
#[derive(Debug, Clone)]
pub struct FeatureRegistry {
    /// A list of all registered world generation features.
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
