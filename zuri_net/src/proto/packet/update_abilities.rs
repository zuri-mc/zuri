use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::ability::AbilityData;

/// Sent from the server to update the abilities of the player. It, along with the
/// UpdateAdventureSettings packet, are replacements of the AdventureSettings packet since v1.19.10.
#[derive(Debug, Clone)]
pub struct UpdateAbilities {
    /// Various data about the abilities of a player, such as ability layers or permissions.
    pub ability_data: AbilityData,
}

impl PacketType for UpdateAbilities {
    fn write(&self, writer: &mut Writer) {
        self.ability_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { ability_data: AbilityData::read(reader) }
    }
}
