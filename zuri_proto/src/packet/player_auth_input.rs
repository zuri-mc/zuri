use glam::{Vec2, Vec3};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};
use crate::types::player::PlayerBlockAction;
use crate::types::inventory::UseItemTransactionData;
use crate::types::item_stack::ItemStackRequestEntry;

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum InputFlag {
    Ascend,
    Descend,
    NorthJump,
    JumpDown,
    SprintDown,
    ChangeHeight,
    Jumping,
    AutoJumpingInWater,
    Sneaking,
    SneakDown,
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    WantUp,
    WantDown,
    WantDownSlow,
    WantUpSlow,
    Sprinting,
    AscendBlock,
    DescendBlock,
    SneakToggleDown,
    PersistSneak,
    StartSprinting,
    StopSprinting,
    StartSneaking,
    StopSneaking,
    StartSwimming,
    StopSwimming,
    StartJumping,
    StartGliding,
    StopGliding,
    PerformItemInteraction,
    PerformBlockActions,
    PerformItemStackRequest,
}

impl InputFlag {
    pub fn flag(&self) -> u64 {
        1 << (*self as u64)
    }
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PlayMode {
    Normal,
    Teaser,
    Screen,
    Viewer,
    Reality,
    Placement,
    LivingRoom,
    ExitLevel,
    ExitLevelLivingRoom,
    NumModes,
}

#[derive(Debug)]
pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: Vec3,
    pub move_vector: Vec2,
    pub head_yaw: f32,
    pub input_data: u64,
    pub input_mode: u32,
    pub play_mode: PlayMode,
    pub interaction_model: i32,
    pub gaze_direction: Vec3,
    pub tick: u64,
    pub delta: Vec3,
    pub item_interaction_data: UseItemTransactionData,
    pub item_stack_request: ItemStackRequestEntry,
    pub block_actions: Vec<PlayerBlockAction>,
}

impl Packet for PlayerAuthInput {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.vec3(self.position);
        writer.vec2(self.move_vector);
        writer.f32(self.head_yaw);
        writer.var_u64(self.input_data);
        writer.var_u32(self.input_mode);
        writer.var_u32(self.play_mode.to_u32().unwrap());
        writer.i32(self.interaction_model);
        if self.play_mode == PlayMode::Reality {
            writer.vec3(self.gaze_direction);
        }
        writer.var_u64(self.tick);
        writer.vec3(self.delta);

        if self.input_data & InputFlag::PerformItemInteraction.flag() != 0 {
            self.item_interaction_data.write_player_action(writer);
        }
        if self.input_data & InputFlag::PerformItemStackRequest.flag() != 0 {
            self.item_stack_request.write(writer);
        }
        if self.input_data & InputFlag::PerformBlockActions.flag() != 0 {
            writer.var_u32(self.block_actions.len() as u32);
            self.block_actions.iter().for_each(|action| action.write(writer));
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            pitch: reader.f32(),
            yaw: reader.f32(),
            position: reader.vec3(),
            move_vector: reader.vec2(),
            head_yaw: reader.f32(),
            input_data: reader.var_u64(),
            input_mode: reader.var_u32(),
            play_mode: PlayMode::from_u32(reader.var_u32()).unwrap(),
            interaction_model: reader.i32(),
            gaze_direction: Vec3::default(),
            tick: reader.var_u64(),
            delta: reader.vec3(),
            item_interaction_data: Default::default(),
            item_stack_request: Default::default(), // todo
            block_actions: Vec::new(),
        };
        if packet.play_mode == PlayMode::Reality {
            reader.vec3();
        }
        if packet.input_data & InputFlag::PerformItemInteraction.flag() != 0 {
            packet.item_interaction_data = UseItemTransactionData::read_player_action(reader);
        }
        if packet.input_data & InputFlag::PerformItemStackRequest.flag() != 0 {
            packet.item_stack_request = ItemStackRequestEntry::read(reader);
        }
        if packet.input_data & InputFlag::PerformBlockActions.flag() != 0 {
            packet.block_actions = (0..reader.var_u32()).map(|_| PlayerBlockAction::read(reader)).collect();
        }
        packet
    }
}
