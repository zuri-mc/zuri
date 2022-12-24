use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct PlayerList {
    pub action_type: PlayerListAction,
    pub entries: Vec<PlayerListEntry>,
}

impl Packet for PlayerList {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());
        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| PlayerListEntry::read(reader, action_type)).collect(),
        }
    }
}
