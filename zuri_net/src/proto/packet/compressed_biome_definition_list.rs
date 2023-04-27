use bytes::Bytes;
use zuri_net_derive::proto;

#[proto]
#[derive(Debug, Clone)]
pub struct CompressedBiomeDefinitionList {
    pub serialised_biome_definitions: Bytes,
}
