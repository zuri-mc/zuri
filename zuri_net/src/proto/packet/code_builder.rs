use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Education Edition packet sent by the server to open the URL to a Code Builder server.
#[derive(Debug, Clone)]
pub struct CodeBuilder {
    /// The URL to the Code Builder (WS) server.
    pub url: String,
    /// Specifies if the client should automatically open the Code Builder app. If set to true, the
    /// client will attempt to use the Code Builder app to connect to and interface with the server
    /// running at the URL above.
    pub should_open_code_builder: bool,
}

impl PacketType for CodeBuilder {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.url.as_str());
        writer.bool(self.should_open_code_builder);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            url: reader.string(),
            should_open_code_builder: reader.bool(),
        }
    }
}
