use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct ServerSettingsRequest {}

impl Packet for ServerSettingsRequest {
    fn write(&self, _: &mut Writer) {}

    fn read(_: &mut Reader) -> Self {
        Self {}
    }
}
