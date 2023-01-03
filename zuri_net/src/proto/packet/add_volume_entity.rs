use glam::IVec3;
use num_traits::{ToPrimitive, FromPrimitive};
use zuri_nbt::{Value, encoding::NetworkLittleEndian};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::world::Dimension;

/// Sends a volume entity's definition and metadata from server to client.
#[derive(Debug, Clone)]
pub struct AddVolumeEntity {
    /// The runtime ID of the volume. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A compound tag of entity metadata, which includes flags and data properties that alter in
    /// particular the way the volume functions or looks.
    pub entity_metadata: Value,
    /// The unique identifier for the volume. It must be of the form 'namespace:name', where
    /// namespace cannot be 'minecraft'.
    pub encoding_identifier: String,
    /// The identifier of a fog definition.
    pub instance_identifier: String,
    /// The volume's bounds. The first value is the minimum bounds, and the second value is the
    /// maximum bounds.
    pub bounds: [IVec3; 2],
    /// The dimension in which the volume exists.
    pub dimension: Dimension,
    /// The engine version the entity is using, for example, '1.17.0'.
    pub engine_version: String,
}

impl PacketType for AddVolumeEntity {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        writer.nbt(&self.entity_metadata, NetworkLittleEndian);
        writer.string(self.encoding_identifier.as_str());
        writer.string(self.instance_identifier.as_str());
        writer.u_block_pos(self.bounds[0]);
        writer.u_block_pos(self.bounds[1]);
        writer.var_i32(self.dimension.to_i32().unwrap());
        writer.string(self.engine_version.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.u64(),
            entity_metadata: reader.nbt(NetworkLittleEndian),
            encoding_identifier: reader.string(),
            instance_identifier: reader.string(),
            bounds: [reader.u_block_pos(), reader.u_block_pos()],
            dimension: Dimension::from_i32(reader.var_i32()).unwrap(),
            engine_version: reader.string(),
        }
    }
}
