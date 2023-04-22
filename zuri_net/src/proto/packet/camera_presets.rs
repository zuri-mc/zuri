use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;
use crate::proto::io::NBT;

/// Gives the client a list of custom camera presets.
#[proto]
#[derive(Debug, Clone)]
pub struct CameraPresets {
    /// A compound tag of the presets]to sent. The structure of this tag is currently unknown.
    pub data: NBT<NetworkLittleEndian>,
}
