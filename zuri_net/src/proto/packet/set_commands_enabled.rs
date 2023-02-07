use zuri_net_derive::proto;

/// Sent by the server to enable or disable the ability to execute commands for the client. If
/// disabled, the client itself will stop the execution of commands.\
#[proto]
#[derive(Debug, Clone)]
pub struct SetCommandsEnabled {
    /// Defines if the commands should be enabled, or if false, disabled.
    pub enabled: bool,
}
