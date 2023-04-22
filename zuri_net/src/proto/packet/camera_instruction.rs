use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;
use crate::proto::io::NBT;

/// Gives a custom camera specific instructions to operate.
#[proto]
#[derive(Debug, Clone)]
pub struct CameraInstruction {
    /// A compound tag of the instructions to sent. The structure of this tag is currently unknown.
    pub data: NBT<NetworkLittleEndian>,
}
