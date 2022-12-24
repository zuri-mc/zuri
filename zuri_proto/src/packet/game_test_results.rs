#[derive(Debug)]
pub struct GameTestResults {
    pub name: String,
    pub succeeded: bool,
    pub error: String,
}

impl Packet for GameTestResults {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.succeeded);
        writer.string(self.error.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            succeeded: reader.bool(),
            error: reader.string(),
        }
    }
}
