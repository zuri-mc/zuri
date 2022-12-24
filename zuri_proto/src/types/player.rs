use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PlayerActionType {
    StartBreak,
    AbortBreak,
    StopBreak,
    GetUpdatedBlock,
    DropItem,
    StartSleeping,
    StopSleeping,
    Respawn,
    Jump,
    StartSprint,
    StopSprint,
    StartSneak,
    StopSneak,
    CreativePlayerDestroyBlock,
    DimensionChangeDone,
    StartGlide,
    StopGlide,
    BuildDenied,
    CrackBreak,
    ChangeSkin,
    SetEnchantmentSeed,
    StartSwimming,
    StopSwimming,
    StartSpinAttack,
    StopSpinAttack,
    StartBuildingBlock,
    PredictDestroyBlock,
    ContinueDestroyBlock,
    StartItemUseOn,
    StopItemUseOn,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PlayerMovementMode {
    Client,
    Server,
    ServerWithRewind,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MoveMode {
    Normal,
    Reset,
    Teleport,
    Rotation,
}

#[derive(Debug)]
pub struct PlayerMovementSettings {
    pub movement_type: i32,
    pub rewind_history_size: i32,
    pub server_authoritative_block_breaking: bool,
}

impl PlayerMovementSettings {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.movement_type);
        writer.var_i32(self.rewind_history_size);
        writer.bool(self.server_authoritative_block_breaking);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            movement_type: reader.var_i32(),
            rewind_history_size: reader.var_i32(),
            server_authoritative_block_breaking: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct PlayerBlockAction {
    pub action: PlayerActionType,
    pub block_pos: IVec3,
    pub face: i32,
}

impl PlayerBlockAction {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.action).unwrap());
        match self.action {
            PlayerActionType::StartBreak | PlayerActionType::AbortBreak | PlayerActionType::CrackBreak | PlayerActionType::PredictDestroyBlock | PlayerActionType::ContinueDestroyBlock => {
                writer.block_pos(self.block_pos);
                writer.var_i32(self.face);
            }
            _ => {}
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let mut action = Self {
            action: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            block_pos: IVec3::default(),
            face: 0,
        };
        match action.action {
            PlayerActionType::StartBreak | PlayerActionType::AbortBreak | PlayerActionType::CrackBreak | PlayerActionType::PredictDestroyBlock | PlayerActionType::ContinueDestroyBlock => {
                action.block_pos = reader.block_pos();
                action.face = reader.var_i32();
            }
            _ => {}
        }

        action
    }
}
