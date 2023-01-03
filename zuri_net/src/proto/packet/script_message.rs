use bytes::Bytes;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Used to communicate custom messages from the client to the server, or from the server to the
/// client. While the name may suggest this packet is used for the discontinued scripting API, it is
/// likely instead for the GameTest framework.
#[derive(Debug, Clone)]
pub struct ScriptMessage {
    /// The identifier of the message, used by either party to identify the message data sent.
    pub identifier: String,
    /// The data of the message.
    pub data: Bytes,
}

impl PacketType for ScriptMessage {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.identifier.as_str());
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            identifier: reader.string(),
            data: reader.byte_slice(),
        }
    }
}
