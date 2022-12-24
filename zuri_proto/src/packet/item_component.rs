#[derive(Debug)]
pub struct ItemComponent {
    pub items: Vec<ItemComponentEntry>,
}

impl Packet for ItemComponent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| ItemComponentEntry::read(reader)).collect() }
    }
}
