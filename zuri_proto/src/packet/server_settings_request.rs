use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct ServerSettingsRequest {}

impl PacketType for ServerSettingsRequest {
    fn write(&self, _: &mut Writer) {}

    fn read(_: &mut Reader) -> Self {
        Self {}
    }
}
