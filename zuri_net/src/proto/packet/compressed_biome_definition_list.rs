use bytes::Bytes;
use zuri_net_derive::proto;

/// Sent by the server when client-side generation is enabled. It is similar to
/// [CompressedBiomeDefinitionList](super::biome_definition_list::BiomeDefinitionList) but contains
/// extra data so the client can generate chunks locally.
#[proto]
#[derive(Debug, Clone)]
pub struct CompressedBiomeDefinitionList {
    /// Compressed NBT data that contains all biome definitions.
    pub serialised_biome_definitions: Bytes,
}
