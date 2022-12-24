#[derive(Debug)]
pub struct NetworkChunkPublisherUpdate {
    pub position: BlockPos,
    pub radius: u32,
    pub saved_chunks: Vec<IVec2>,
}

impl Packet for NetworkChunkPublisherUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.var_u32(self.radius);
        writer.u32(self.saved_chunks.len() as u32);
        self.saved_chunks.iter().for_each(|chunk| {
            writer.var_i32(chunk.x);
            writer.var_i32(chunk.y);
        });
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            radius: reader.var_u32(),
            saved_chunks: (0..reader.u32()).map(|_| IVec2::new(reader.var_i32(), reader.var_i32())).collect(),
        }
    }
}
