use zuri_nbt::{encoding::NetworkLittleEndian, Value};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent from the server to the client and vise-versa to communicate editor-mode related
/// information. It carries a single compound tag containing the relevant information.
#[derive(Debug, Clone)]
pub struct EditorNetwork {
    /// A network little endian compound tag holding data relevant to the editor.
    pub payload: Value,
}

impl PacketType for EditorNetwork {
    fn write(&self, writer: &mut Writer) {
        writer.nbt(&self.payload, NetworkLittleEndian);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { payload: reader.nbt(NetworkLittleEndian) }
    }
}
