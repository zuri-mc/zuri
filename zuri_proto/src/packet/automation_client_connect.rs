#[derive(Debug)]
pub struct AutomationClientConnect {
    pub server_uri: String,
}

impl Packet for AutomationClientConnect {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.server_uri.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            server_uri: reader.string(),
        }
    }
}
