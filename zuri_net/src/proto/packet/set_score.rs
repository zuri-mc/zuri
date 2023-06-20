use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::scoreboard::{ScoreboardAction, ScoreboardEntry};

/// Sent by the server to send the contents of a scoreboard to the player. It may be used to either
/// add, remove or edit entries on the scoreboard.
#[derive(Debug, Clone)]
pub struct SetScore {
    /// The type of the action to execute upon the scoreboard with the entries that the packet has.
    /// If `action_type` is `Modify`, all entries will be added to the scoreboard if not yet
    /// present, or modified if already present. If set to `Remove`, all scoreboard entries set will
    /// be removed from the scoreboard.
    pub action_type: ScoreboardAction,
    /// A list of all entries that the client should operate on. When modifying, it will add or
    /// modify all entries, whereas when removing, it will remove all entries.
    pub entries: Vec<ScoreboardEntry>,
}

impl PacketType for SetScore {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries
            .iter()
            .for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = ScoreboardAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32())
                .map(|_| ScoreboardEntry::read(reader, action_type))
                .collect(),
        }
    }
}
