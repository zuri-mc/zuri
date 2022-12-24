#[derive(Debug)]
pub struct MultiPlayerSettings {
    pub action_type: i32,
}

impl Packet for MultiPlayerSettings {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_i32(),
        }
    }
}
