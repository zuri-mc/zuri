#[derive(Debug)]
pub struct ChunkRadiusUpdated {
    pub chunk_radius: i32,
}

impl Packet for ChunkRadiusUpdated {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.chunk_radius);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            chunk_radius: reader.var_i32(),
        }
    }
}
