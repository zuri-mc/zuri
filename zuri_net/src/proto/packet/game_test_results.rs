use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent in response to the GameTestRequest packet, with a boolean indicating whether the test was
/// successful or not, and an error string if the test failed.
#[derive(Debug, Clone)]
pub struct GameTestResults {
    /// The name of the test.
    pub name: String,
    /// Indicates whether the test succeeded or not.
    pub succeeded: bool,
    /// The error that occurred. If succeeded is true, this field is empty.
    pub error: String,
}

impl PacketType for GameTestResults {
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
