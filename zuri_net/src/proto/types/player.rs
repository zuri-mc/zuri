use crate::proto::ints::VarI32;
use glam::IVec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use zuri_net_derive::proto;

use crate::proto::io::{Reader, Writer};

#[proto(VarI32)]
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
    HandledTeleport,
    MissedSwing,
    StartCrawling,
    StopCrawling,
    StartFlying,
    StopFlying,
    ClientAckServerData,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PlayerMovementMode {
    Client,
    Server,
    ServerWithRewind,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MoveMode {
    Normal,
    Reset,
    Teleport,
    Rotation,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum InputMode {
    None,
    Mouse,
    Touch,
    GamePad,
    MotionController,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum InteractionModel {
    Touch,
    Crosshair,
    Classic,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum TeleportCause {
    None,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PlayerMovementSettings {
    pub movement_type: VarI32,
    pub rewind_history_size: VarI32,
    pub server_authoritative_block_breaking: bool,
}

#[derive(Debug, Clone)]
pub struct PlayerBlockAction {
    pub action: PlayerActionType,
    pub block_pos: IVec3,
    pub face: i32,
}

impl PlayerBlockAction {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action.to_i32().unwrap());
        match self.action {
            PlayerActionType::StartBreak
            | PlayerActionType::AbortBreak
            | PlayerActionType::CrackBreak
            | PlayerActionType::PredictDestroyBlock
            | PlayerActionType::ContinueDestroyBlock => {
                writer.block_pos(self.block_pos);
                writer.var_i32(self.face);
            }
            _ => {}
        }
    }

    pub fn read(reader: &mut Reader) -> Self {
        let mut action = Self {
            action: PlayerActionType::from_i32(reader.var_i32()).unwrap(),
            block_pos: IVec3::default(),
            face: 0,
        };
        match action.action {
            PlayerActionType::StartBreak
            | PlayerActionType::AbortBreak
            | PlayerActionType::CrackBreak
            | PlayerActionType::PredictDestroyBlock
            | PlayerActionType::ContinueDestroyBlock => {
                action.block_pos = reader.block_pos();
                action.face = reader.var_i32();
            }
            _ => {}
        }

        action
    }
}
