#[derive(Debug)]
pub struct TickingAreasLoadStatus {
    pub preload: bool,
}

impl Packet for TickingAreasLoadStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.preload);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            preload: reader.bool(),
        }
    }
}
