use crate::io::{Reader, Writer};
use crate::packet::PacketType;
use crate::types::ability::AbilityData;

#[derive(Debug)]
pub struct UpdateAbilities {
    pub ability_data: AbilityData,
}

impl PacketType for UpdateAbilities {
    fn write(&self, writer: &mut Writer) {
        self.ability_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            ability_data: AbilityData::read(reader),
        }
    }
}
