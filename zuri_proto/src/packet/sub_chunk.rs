#[derive(Debug)]
pub struct SubChunk {
    pub cache_enabled: bool,
    pub dimension: Dimension,
    pub position: BlockPos,
    pub sub_chunk_entries: Vec<SubChunkEntry>,
}

impl Packet for SubChunk {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.cache_enabled);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.block_pos(self.position);
        writer.u32(self.sub_chunk_entries.len() as u32);
        self.sub_chunk_entries.iter().for_each(|entry| entry.write(writer, self.cache_enabled));
    }

    fn read(reader: &mut Reader) -> Self {
        let cache_enabled = reader.bool();
        Self {
            cache_enabled,
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            sub_chunk_entries: (0..reader.u32()).map(|_| SubChunkEntry::read(reader, cache_enabled)).collect(),
        }
    }
}
