use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct RequestChunkRadius {
    pub chunk_radius: i32,
}

impl Packet for RequestChunkRadius {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.chunk_radius);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            chunk_radius: reader.var_i32(),
        }
    }
}
