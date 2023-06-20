use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;

use crate::proto::io::{UBlockPos, NBT};
use crate::proto::types::world::Dimension;

/// Sends a volume entity's definition and metadata from server to client.
#[proto]
#[derive(Debug, Clone)]
pub struct AddVolumeEntity {
    /// The runtime ID of the volume. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A compound tag of entity metadata, which includes flags and data properties that alter in
    /// particular the way the volume functions or looks.
    pub entity_metadata: NBT<NetworkLittleEndian>,
    /// The unique identifier for the volume. It must be of the form 'namespace:name', where
    /// namespace cannot be 'minecraft'.
    pub encoding_identifier: String,
    /// The identifier of a fog definition.
    pub instance_identifier: String,
    /// The volume's bounds. The first value is the minimum bounds, and the second value is the
    /// maximum bounds.
    pub bounds: [UBlockPos; 2],
    /// The dimension in which the volume exists.
    pub dimension: Dimension,
    /// The engine version the entity is using, for example, '1.17.0'.
    pub engine_version: String,
}
