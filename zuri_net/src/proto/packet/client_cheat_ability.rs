use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::ability::AbilityData;

#[derive(Debug, Clone)]
pub struct ClientCheatAbility {
    /// Various data about the abilities of a player, such as ability layers or permissions.
    pub ability_data: AbilityData,
}

impl PacketType for ClientCheatAbility {
    fn write(&self, writer: &mut Writer) {
        self.ability_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { ability_data: AbilityData::read(reader) }
    }
}
