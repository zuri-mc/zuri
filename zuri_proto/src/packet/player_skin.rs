#[derive(Debug)]
pub struct PlayerSkin {
    pub uuid: Uuid,
    pub skin: Skin,
    pub new_skin_name: String,
    pub old_skin_name: String,
}

impl Packet for PlayerSkin {
    fn write(&self, writer: &mut Writer) {
        writer.uuid(self.uuid);
        self.skin.write(writer);
        writer.string(self.new_skin_name.as_str());
        writer.string(self.old_skin_name.as_str());
        writer.bool(self.skin.trusted);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            uuid: reader.uuid(),
            skin: Skin::read(reader),
            new_skin_name: reader.string(),
            old_skin_name: reader.string(),
        };
        packet.skin.trusted = reader.bool();

        packet
    }
}
