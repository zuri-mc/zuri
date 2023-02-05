use crate::proto::io::NBT;
use zuri_net_derive::packet;
use zuri_nbt::encoding::NetworkLittleEndian;

/// Sent from the server to the client and vise-versa to communicate editor-mode related
/// information. It carries a single compound tag containing the relevant information.
#[packet]
#[derive(Debug, Clone)]
pub struct EditorNetwork {
    /// A network little endian compound tag holding data relevant to the editor.
    pub payload: NBT<NetworkLittleEndian>,
}
