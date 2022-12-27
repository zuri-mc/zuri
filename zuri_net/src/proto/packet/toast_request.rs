use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct ToastRequest {
    pub title: String,
    pub message: String,
}

impl PacketType for ToastRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.title.as_str());
        writer.string(self.message.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            title: reader.string(),
            message: reader.string(),
        }
    }
}
