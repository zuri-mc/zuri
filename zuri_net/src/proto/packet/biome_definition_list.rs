use zuri_nbt::encoding::NetworkLittleEndian;

use zuri_net_derive::proto;
use crate::proto::io::NBT;

/// Sent by the server to let the client know all biomes that are available and implemented on the
/// server side. It is much like the AvailableActorIdentifiers packet, but instead for biomes.
#[proto]
#[derive(Debug, Clone)]
pub struct BiomeDefinitionList {
    /// Network NBT serialised tag of all definitions of biomes that are available on the server.
    pub serialised_biome_definitions: NBT<NetworkLittleEndian>,
}
