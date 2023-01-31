use glam::IVec3;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::container::ContainerType;
use crate::proto::types::inventory::Window;

/// Sent by the server to open a container client-side. This container must be physically present in
/// the world, for the packet to have any effect. Unlike Java Edition, Bedrock Edition requires that
/// chests for example must be present and in range to open its inventory.
#[derive(Debug, Clone)]
pub struct ContainerOpen {
    /// The window that is being opened. It may be used later to close the container using a
    /// ContainerClose packet.
    pub window: Window,
    /// The type of the container that is being opened when opening the container at the position of
    /// the packet. It depends on the block/entity, and could, for example, be a chest or a hopper,
    /// but also a horse inventory.
    pub container_type: ContainerType,
    /// The position of the container opened. The position must point to a block entity that
    /// actually has a container. If that is not the case, the window will not be opened and the
    /// packet will be ignored, if a valid container entity unique id has not also been provided.
    pub container_position: IVec3,
    /// The unique ID of the entity container that was opened. It is only used if the ContainerType
    /// is one that points to an entity, for example a horse.
    pub container_entity_unique_id: i64,
}

impl PacketType for ContainerOpen {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.window.to_u8().unwrap());
        writer.u8(self.container_type.to_u8().unwrap());
        writer.u_block_pos(self.container_position);
        writer.var_i64(self.container_entity_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: Window::from_u8(reader.u8()).unwrap(),
            container_type: ContainerType::from_u8(reader.u8()).unwrap(),
            container_position: reader.u_block_pos(),
            container_entity_unique_id: reader.var_i64(),
        }
    }
}
