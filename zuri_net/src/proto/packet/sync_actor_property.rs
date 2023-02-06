use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::packet;

use crate::proto::io::NBT;

/// An alternative to synced actor data. It is not exactly clear how it functions.
#[packet]
#[derive(Debug, Clone)]
pub struct SyncActorProperty {
    /// The purpose of this field is unknown.
    pub property_data: NBT<NetworkLittleEndian>,
}
