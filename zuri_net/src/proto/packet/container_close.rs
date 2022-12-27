use num_traits::{ToPrimitive, FromPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::inventory::Window;

/// Sent by the server to close a container the player currently has opened, which was opened using the ContainerOpen
/// packet, or by the client to tell the server it closed a particular container, such as the crafting grid.
#[derive(Debug, Clone)]
pub struct ContainerClose {
    /// The window of the container that should be closed. It must be equal to the one sent in the ContainerOpen packet
    /// to close the designated window.
    pub window: Window,
    /// Determines whether or not the container was force-closed by the server. If this value is not set correctly, the
    /// client may ignore the packet and respond with a PacketViolationWarning.
    pub server_side: bool,
}

impl PacketType for ContainerClose {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.window.to_u8().unwrap());
        writer.bool(self.server_side);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u8(reader.u8()).unwrap(),
            server_side: reader.bool(),
        }
    }
}
