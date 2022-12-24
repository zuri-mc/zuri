#[derive(Debug)]
pub struct ShowCredits {
    pub player_runtime_id: u64,
    pub status_type: i32,
}

impl Packet for ShowCredits {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.player_runtime_id);
        writer.var_i32(self.status_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_runtime_id: reader.var_u64(),
            status_type: reader.var_i32(),
        }
    }
}
