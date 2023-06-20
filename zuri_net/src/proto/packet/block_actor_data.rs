use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;

use crate::proto::io::{UBlockPos, NBT};

/// Sent by the server to update data of a block entity, for example the data of a chest.
#[proto]
#[derive(Debug, Clone)]
pub struct BlockActorData {
    /// The position of the block that holds the block entity. If no block entity is at this
    /// position, the packet is ignored by the client.
    pub position: UBlockPos,
    /// The new data of the block that will be encoded to NBT and applied client-side, so that the
    /// client can see the block update. The NBTData should contain all properties of the block, not
    /// just properties that were changed.
    pub nbt_data: NBT<NetworkLittleEndian>,
}
