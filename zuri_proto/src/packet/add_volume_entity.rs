#[derive(Debug)]
pub struct AddVolumeEntity {
    pub entity_runtime_id: u64,
    //pub entity_metadata: dyn Any,
    // TODO: NBT
    pub encoding_identifier: String,
    pub instance_identifier: String,
    pub bounds: [BlockPos; 2],
    pub dimension: Dimension,
    pub engine_version: String,
}

impl Packet for AddVolumeEntity {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        // TODO: NBT (entity_metadata)(
        writer.string(self.encoding_identifier.as_str());
        writer.string(self.instance_identifier.as_str());
        writer.u_block_pos(self.bounds[0]);
        writer.u_block_pos(self.bounds[1]);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.string(self.engine_version.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.u64(),
            // entity_metadata: {
            //     // TODO: NBT
            // },
            encoding_identifier: reader.string(),
            instance_identifier: reader.string(),
            bounds: [reader.u_block_pos(), reader.u_block_pos()],
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            engine_version: reader.string(),
        }
    }
}
