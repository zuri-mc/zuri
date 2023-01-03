use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent from the server to update the adventure settings of the player. It, along with the
/// UpdateAbilities packet, are replacements of the AdventureSettings packet since v1.19.10.
#[derive(Debug, Clone)]
pub struct UpdateAdventureSettings {
    /// If the player is allowed to fight mobs.
    pub no_pvm: bool,
    /// If mobs are allowed to fight the player. It is unclear why this is sent to the client, as
    /// only the server needs to know this.
    pub no_mvp: bool,
    /// If the player is allowed to modify the world.
    pub immutable_world: bool,
    /// If name-tags are shown.
    pub show_name_tags: bool,
    /// If the player is allowed to jump automatically.
    pub auto_jump: bool,
}

impl PacketType for UpdateAdventureSettings {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.no_pvm);
        writer.bool(self.no_mvp);
        writer.bool(self.immutable_world);
        writer.bool(self.show_name_tags);
        writer.bool(self.auto_jump);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            no_pvm: reader.bool(),
            no_mvp: reader.bool(),
            immutable_world: reader.bool(),
            show_name_tags: reader.bool(),
            auto_jump: reader.bool(),
        }
    }
}
