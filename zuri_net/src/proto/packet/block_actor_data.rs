use glam::IVec3;
use zuri_nbt::{Value, encoding::NetworkLittleEndian};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to update data of a block entity, for example the data of a chest.
#[derive(Debug, Clone)]
pub struct BlockActorData {
    /// The position of the block that holds the block entity. If no block entity is at this
    /// position, the packet is ignored by the client.
    pub position: IVec3,
    /// The new data of the block that will be encoded to NBT and applied client-side, so that the
    /// client can see the block update. The NBTData should contain all properties of the block, not
    /// just properties that were changed.
    pub nbt_data: Value,
}

impl PacketType for BlockActorData {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.nbt(&self.nbt_data, NetworkLittleEndian);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            nbt_data: reader.nbt(NetworkLittleEndian),
        }
    }
}
