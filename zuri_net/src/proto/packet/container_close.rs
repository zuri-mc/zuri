use zuri_net_derive::proto;

use crate::proto::types::inventory::Window;

/// Sent by the server to close a container the player currently has opened, which was opened using
/// the ContainerOpen packet, or by the client to tell the server it closed a particular container,
/// such as the crafting grid.
#[proto]
#[derive(Debug, Clone)]
pub struct ContainerClose {
    /// The window of the container that should be closed. It must be equal to the one sent in the
    /// ContainerOpen packet to close the designated window.
    pub window: Window,
    /// Determines whether or not the container was force-closed by the server. If this value is not
    /// set correctly, the client may ignore the packet and respond with a PacketViolationWarning.
    pub server_side: bool,
}
