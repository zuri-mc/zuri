#[derive(Debug)]
pub struct ClientCacheStatus {
    pub enabled: bool,
}

impl Packet for ClientCacheStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.enabled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            enabled: reader.bool(),
        }
    }
}
