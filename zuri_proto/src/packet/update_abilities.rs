#[derive(Debug)]
pub struct UpdateAbilities {
    pub ability_data: AbilityData,
}

impl Packet for UpdateAbilities {
    fn write(&self, writer: &mut Writer) {
        self.ability_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            ability_data: AbilityData::read(reader),
        }
    }
}
