use zuri_net_derive::packet;

/// Sent in response to the GameTestRequest packet, with a boolean indicating whether the test was
/// successful or not, and an error string if the test failed.
#[packet]
#[derive(Debug, Clone)]
pub struct GameTestResults {
    /// The name of the test.
    pub name: String,
    /// Indicates whether the test succeeded or not.
    pub succeeded: bool,
    /// The error that occurred. If succeeded is true, this field is empty.
    pub error: String,
}
