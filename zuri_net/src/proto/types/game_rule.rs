use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone)]
pub struct GameRule {
    pub name: String,
    pub can_be_modified_by_player: bool,
    pub value: GameRuleValue,
}

#[derive(Debug, Clone)]
pub enum GameRuleValue {
    Bool(bool),
    Int(u32),
    Float(f32),
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
enum GameRuleType {
    Bool = 1,
    Int = 2,
    Float = 3,
}

impl GameRule {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.can_be_modified_by_player);
        match self.value {
            GameRuleValue::Bool(value) => {
                writer.var_u32(GameRuleType::Bool.to_u32().unwrap());
                writer.bool(value);
            }
            GameRuleValue::Int(value) => {
                writer.var_u32(GameRuleType::Int.to_u32().unwrap());
                writer.var_u32(value);
            }
            GameRuleValue::Float(value) => {
                writer.var_u32(GameRuleType::Float.to_u32().unwrap());
                writer.f32(value);
            }
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            can_be_modified_by_player: reader.bool(),
            value: match GameRuleType::from_u32(reader.var_u32()).unwrap() {
                GameRuleType::Bool => GameRuleValue::Bool(reader.bool()),
                GameRuleType::Int => GameRuleValue::Int(reader.var_u32()),
                GameRuleType::Float => GameRuleValue::Float(reader.f32()),
            },
        }
    }
}
