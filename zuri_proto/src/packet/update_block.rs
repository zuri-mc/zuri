use crate::io::{Reader, Writer};
use crate::packet::Packet;

/// Sent by the server to update a block client-side, without resending the entire chunk that the block is located in.
/// It is particularly useful for small modifications like block breaking/placing.
#[derive(Debug)]
pub struct UpdateBlock {
    /// The block position at which a block is updated.
    pub position: BlockPos,
    /// The runtime ID of the block that is placed at position after sending the packet to the client.
    pub new_block_runtime_id: u32,
    /// A combination of BlockUpdate flags that specify the way the block is updated client-side. Typically, sending
    /// only the Network flag is sufficient.
    pub flags: u32,
    /// The world layer on which the block is updated. For most blocks, this is the first layer, as that layer is the
    /// default layer to place blocks on, but for blocks inside of each other, this differs.
    pub layer: u32,
}

impl Packet for UpdateBlock {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_u32(self.new_block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u32(self.layer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            new_block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            layer: reader.var_u32(),
        }
    }
}
