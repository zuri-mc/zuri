#[derive(Debug)]
pub struct UpdateClientInputLocks {
    pub locks: u32,
    pub position: Vec3,
}

impl Packet for UpdateClientInputLocks {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.locks);
        writer.vec3(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            locks: reader.var_u32(),
            position: reader.vec3(),
        }
    }
}
