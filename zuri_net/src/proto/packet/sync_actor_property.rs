use zuri_nbt::{encoding::NetworkLittleEndian, Value};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// An alternative to synced actor data. It is not exactly clear how it functions.
#[derive(Debug, Clone)]
pub struct SyncActorProperty {
    /// The purpose of this field is unknown.
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
