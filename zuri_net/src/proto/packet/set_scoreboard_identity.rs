use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::scoreboard::{ScoreboardIdentityAction, ScoreboardIdentityEntry};

/// Sent by the server to change the identity type of one of the entries on a scoreboard. This is
/// used to change, for example, an entry pointing to a player, to a fake player when it leaves the
/// server, and to change it back to a real player when it joins again. In non-vanilla situations,
/// the packet is quite useless.
#[derive(Debug, Clone)]
pub struct SetScoreboardIdentity {
    /// The type of the action to execute. The action is either `Register` to associate an identity
    /// with the entry, or `Clear` to remove associations with an entity.
    pub action_type: ScoreboardIdentityAction,
    /// A list of all entries in the packet. Each of these entries points to one of the entries on
    /// a scoreboard. Depending on `action_type`, it'll either be registered or cleared.
    pub entries: Vec<ScoreboardIdentityEntry>,
}

impl PacketType for SetScoreboardIdentity {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries
            .iter()
            .for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = ScoreboardIdentityAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32())
                .map(|_| ScoreboardIdentityEntry::read(reader, action_type))
                .collect(),
        }
    }
}
