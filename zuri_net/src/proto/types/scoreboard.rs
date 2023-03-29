use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use zuri_net_derive::proto;

use crate::proto::io::{Reader, Writer};

#[proto(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ScoreboardAction {
    Modify,
    Remove,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ScoreboardIdentity {
    Player = 1,
    Entity = 2,
    FakePlayer = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ScoreboardIdentityAction {
    Register,
    Clear,
}

#[derive(Debug, Clone)]
pub enum ScoreboardSlot {
    List,
    Sidebar,
    BelowName,
}

impl ScoreboardSlot {
    pub fn from_string(s: &str) -> Option<ScoreboardSlot> {
        match s {
            "list" => Some(ScoreboardSlot::List),
            "sidebar" => Some(ScoreboardSlot::Sidebar),
            "belowname" => Some(ScoreboardSlot::BelowName),
            _ => None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            ScoreboardSlot::List => "list",
            ScoreboardSlot::Sidebar => "sidebar",
            ScoreboardSlot::BelowName => "belowname",
        }
    }
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ScoreboardSortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone)]
pub struct ScoreboardEntry {
    pub entry_id: i64,
    pub objective_name: String,
    pub score: i32,
    pub identity_type: ScoreboardIdentity,
    pub entity_unique_id: i64,
    pub display_name: String,
}

impl ScoreboardEntry {
    pub fn write(&self, writer: &mut Writer, action: ScoreboardAction) {
        writer.var_i64(self.entry_id);
        writer.string(self.objective_name.as_str());
        writer.i32(self.score);
        if action == ScoreboardAction::Modify {
            writer.u8(self.identity_type.to_u8().unwrap());
            match self.identity_type {
                ScoreboardIdentity::Entity | ScoreboardIdentity::Player => {
                    writer.var_i64(self.entity_unique_id);
                }
                _ => {
                    writer.string(self.display_name.as_str());
                }
            }
        }
    }

    pub fn read(reader: &mut Reader, action: ScoreboardAction) -> Self {
        let mut entry = Self {
            entry_id: reader.var_i64(),
            objective_name: reader.string(),
            score: reader.i32(),
            identity_type: ScoreboardIdentity::Player,
            display_name: String::new(),
            entity_unique_id: 0,
        };
        if action == ScoreboardAction::Modify {
            entry.identity_type = ScoreboardIdentity::from_u8(reader.u8()).unwrap();
            match entry.identity_type {
                ScoreboardIdentity::Entity | ScoreboardIdentity::Player => {
                    entry.entity_unique_id = reader.var_i64();
                }
                _ => {
                    entry.display_name = reader.string();
                }
            }
        }

        entry
    }
}

#[derive(Debug, Clone)]
pub struct ScoreboardIdentityEntry {
    pub entry_id: i64,
    pub entity_unique_id: i64,
}

impl ScoreboardIdentityEntry {
    pub fn write(&self, writer: &mut Writer, action: ScoreboardIdentityAction) {
        writer.var_i64(self.entry_id);
        if action == ScoreboardIdentityAction::Register {
            writer.var_i64(self.entity_unique_id);
        }
    }

    pub fn read(reader: &mut Reader, action: ScoreboardIdentityAction) -> Self {
        Self {
            entry_id: reader.var_i64(),
            entity_unique_id: if action == ScoreboardIdentityAction::Register {
                reader.var_i64()
            } else {
                0
            },
        }
    }
}
