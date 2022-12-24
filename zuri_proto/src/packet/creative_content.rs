#[derive(Debug)]
pub struct CreativeContent {
    pub items: Vec<CreativeItem>,
}

impl Packet for CreativeContent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| CreativeItem::read(reader)).collect() }
    }
}
