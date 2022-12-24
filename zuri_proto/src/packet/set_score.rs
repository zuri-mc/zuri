use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};
use crate::types::scoreboard::{ScoreboardAction, ScoreboardEntry};

#[derive(Debug)]
pub struct SetScore {
    pub action_type: ScoreboardAction,
    pub entries: Vec<ScoreboardEntry>,
}

impl PacketType for SetScore {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = ScoreboardAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| ScoreboardEntry::read(reader, action_type)).collect(),
        }
    }
}
