use glam::IVec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone)]
pub struct UpdateBlockSynced {
    pub position: IVec3,
    pub new_block_runtime_id: u32,
    pub flags: u32,
    pub layer: u32,
    pub entity_unique_id: i64,
    pub transition_type: u64,
}

impl PacketType for UpdateBlockSynced {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_u32(self.new_block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u32(self.layer);
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.transition_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            new_block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            layer: reader.var_u32(),
            entity_unique_id: reader.var_i64(),
            transition_type: reader.var_u64(),
        }
    }
}
