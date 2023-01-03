use glam::IVec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the client to request the dealing damage to an anvil. This packet is completely
/// pointless and the server should never listen to it.
#[derive(Debug, Clone)]
pub struct AnvilDamage {
    /// The damage that the client requests to be dealt to the anvil.
    pub damage: u8,
    /// The position in the world that the anvil can be found at.
    pub anvil_position: IVec3,
}

impl PacketType for AnvilDamage {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.damage);
        writer.u_block_pos(self.anvil_position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            damage: reader.u8(),
            anvil_position: reader.u_block_pos(),
        }
    }
}
