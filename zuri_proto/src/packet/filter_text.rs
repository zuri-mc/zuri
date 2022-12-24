use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct FilterText {
    pub text: String,
    pub from_server: bool,
}

impl PacketType for FilterText {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.text.as_str());
        writer.bool(self.from_server);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            text: reader.string(),
            from_server: reader.bool(),
        }
    }
}
