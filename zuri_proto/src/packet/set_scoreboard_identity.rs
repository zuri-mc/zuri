use num_traits::{FromPrimitive, ToPrimitive};

use crate::io::{Reader, Writer};
use crate::packet::PacketType;
use crate::types::scoreboard::{ScoreboardIdentityAction, ScoreboardIdentityEntry};

#[derive(Debug)]
pub struct SetScoreboardIdentity {
    pub action_type: ScoreboardIdentityAction,
    pub entries: Vec<ScoreboardIdentityEntry>,
}

impl PacketType for SetScoreboardIdentity {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = ScoreboardIdentityAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| ScoreboardIdentityEntry::read(reader, action_type)).collect(),
        }
    }
}
