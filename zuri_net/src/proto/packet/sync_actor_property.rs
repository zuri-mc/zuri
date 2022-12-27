use zuri_nbt::{Value, encoding::NetworkLittleEndian};
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug)]
pub struct SyncActorProperty {
    pub property_data: Value,
}

impl PacketType for SyncActorProperty {
    fn write(&self, writer: &mut Writer) {
        writer.nbt(&self.property_data, NetworkLittleEndian);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { property_data: reader.nbt(NetworkLittleEndian) }
    }
}
