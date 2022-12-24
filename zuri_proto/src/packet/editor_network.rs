use zuri_nbt::{Value, encoding::NetworkLittleEndian};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct EditorNetwork {
    pub payload: Value,
}

impl Packet for EditorNetwork {
    fn write(&self, writer: &mut Writer) {
        writer.nbt(&self.payload, NetworkLittleEndian);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { payload: reader.nbt(NetworkLittleEndian) }
    }
}
