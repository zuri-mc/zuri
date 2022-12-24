use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct UpdateAdventureSettings {
    pub no_pvm: bool,
    pub no_mvp: bool,
    pub immutable_world: bool,
    pub show_name_tags: bool,
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
