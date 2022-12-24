#[derive(Debug)]
pub struct ItemStackResponse {
    pub responses: Vec<ItemStackResponseEntry>,
}

impl Packet for ItemStackResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.responses.len() as u32);
        self.responses.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { responses: (0..reader.var_u32()).map(|_| ItemStackResponseEntry::read(reader)).collect() }
    }
}
