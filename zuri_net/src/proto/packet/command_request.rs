use crate::proto::types::command::CommandOrigin;
use zuri_net_derive::proto;

/// Sent by the client to request the execution of a server-side command. Although some servers
/// support sending commands using the Text packet, this packet is guaranteed to have the correct
/// result.
#[proto]
#[derive(Debug, Clone)]
pub struct CommandRequest {
    /// The raw entered command line. The client does no parsing of the command line by itself
    /// (unlike it did in the early stages), but lets the server do that.
    pub command_line: String,
    /// The data specifying the origin of the command. In other words, the source that the command
    /// was from, such as the player itself or a WS server.
    pub command_origin: CommandOrigin,
    /// Specifies if the command request internal. Setting it to false seems to work and the usage
    /// of this field is not known.
    pub internal: bool,
    /// The version of the command that is being executed. This field currently has no purpose or
    /// functionality.
    pub version: i32,
}
