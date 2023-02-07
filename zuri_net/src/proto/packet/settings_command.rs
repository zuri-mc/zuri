use zuri_net_derive::proto;

/// Sent by the client when it changes a setting in the settings that results in the issuing of a
/// command to the server, such as when Show Coordinates is enabled.
#[proto]
#[derive(Debug, Clone)]
pub struct SettingsCommand {
    /// The full command line that was sent to the server as a result of the setting that the client
    /// changed.
    pub command_line: String,
    /// Specifies if the client requests the suppressing of the output of the command that was
    /// executed. Generally this is set to true, as the client won't need a message to confirm the
    /// output of the change.
    pub suppress_output: bool,
}
