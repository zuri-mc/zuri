use glam::{Vec2, Vec3};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::inventory::UseItemTransactionData;
use crate::proto::types::item_stack::ItemStackRequestEntry;
use crate::proto::types::player::{InputMode, InteractionModel, PlayerBlockAction};

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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
    HandledTeleport,
    Emoting,
    StartFlying,
    StopFlying,
    ClientAckServerData,
}

impl InputFlag {
    pub fn flag(&self) -> u64 {
        1 << (*self as u64)
    }
}

/// Sent by the client to allow for server authoritative movement. It is used to synchronise the
/// player input with the position server-side. The client sends this packet when the server
/// authoritative movement mode field in the StartGame packet is set to true. Instead of the
/// MovePlayer packet, the client will send this packet once every tick.
#[derive(Debug, Clone)]
pub struct PlayerAuthInput {
    /// The pitch the player reports it has.
    pub pitch: f32,
    /// The yaw the player reports it has.
    pub yaw: f32,
    /// The position that the player reports it has.
    pub position: Vec3,
    /// A Vec2 that specifies the direction in which the player moved, as a combination of X/Z
    /// values which are created using the WASD/controller stick state.
    pub move_vector: Vec2,
    /// The horizontal rotation of the head that the player reports it has.
    pub head_yaw: f32,
    /// A combination of bit flags that together specify the way the player moved last tick.
    pub input_data: u64,
    /// Specifies the way that the client inputs data to the screen.
    pub input_mode: InputMode,
    /// Specifies the way that the player is playing. The values it holds, which are rather random,
    /// may be found above.
    pub play_mode: PlayMode,
    /// The interaction model the player is using.
    pub interaction_model: InteractionModel,
    /// The direction in which the player is gazing, when the `play_mode` is reality. In other
    /// words, when the player is playing in virtual reality.
    pub gaze_direction: Vec3,
    /// The server tick at which the packet was sent. It is used in relation to the
    /// CorrectPlayerMovePrediction packet.
    pub tick: u64,
    /// The delta between the old and the new position. There isn't any practical use for this field
    /// as it can be calculated by the server itself.
    pub delta: Vec3,
    /// The transaction data if the `input_data` includes an item interaction.
    pub item_interaction_data: UseItemTransactionData,
    /// Sent by the client to change an item in their inventory.
    pub item_stack_request: ItemStackRequestEntry,
    /// A list of block actions that the client has interacted with.
    pub block_actions: Vec<PlayerBlockAction>,
    /// The direction in which the player moved, as a combination of X/Z values which are created
    /// using an analogue input.
    pub analogue_move_vector: Vec2,
}

impl PacketType for PlayerAuthInput {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.vec3(self.position);
        writer.vec2(self.move_vector);
        writer.f32(self.head_yaw);
        writer.var_u64(self.input_data);
        writer.var_u32(self.input_mode.to_u32().unwrap());
        writer.var_u32(self.play_mode.to_u32().unwrap());
        writer.var_i32(self.interaction_model.to_i32().unwrap());
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
            self.block_actions
                .iter()
                .for_each(|action| action.write(writer));
        }
        writer.vec2(self.analogue_move_vector);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            pitch: reader.f32(),
            yaw: reader.f32(),
            position: reader.vec3(),
            move_vector: reader.vec2(),
            head_yaw: reader.f32(),
            input_data: reader.var_u64(),
            input_mode: InputMode::from_u32(reader.var_u32()).unwrap(),
            play_mode: PlayMode::from_u32(reader.var_u32()).unwrap(),
            interaction_model: InteractionModel::from_i32(reader.var_i32()).unwrap(),
            gaze_direction: Vec3::default(),
            tick: reader.var_u64(),
            delta: reader.vec3(),
            item_interaction_data: Default::default(),
            item_stack_request: Default::default(), // todo
            block_actions: Vec::new(),
            analogue_move_vector: Vec2::default(),
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
            packet.block_actions = (0..reader.var_u32())
                .map(|_| PlayerBlockAction::read(reader))
                .collect();
        }
        packet.analogue_move_vector = reader.vec2();
        packet
    }
}
