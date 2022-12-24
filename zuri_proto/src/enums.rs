use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AnimationMode {
    None,
    Layers,
    Blocks,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AttributeModifierOperand {
    Min,
    Max,
    Current,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AttributeModifierOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
    Cap,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ClientInputLock {
    Move,
    Jump,
    Sneak,
    Mount,
    Dismount,
    Rotation,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Device {
    None,
    Android,
    IOS,
    OSX,
    FireOS,
    GearVR,
    Hololens,
    Win10,
    Win32,
    Dedicated,
    TVOS,
    Orbis,
    NX,
    XBOX,
    WP,
    Linux,
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
}

impl InputFlag {
    pub fn flag(&self) -> u64 {
        1 << (*self as u64)
    }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum InputMode {
    None,
    Mouse,
    Touch,
    GamePad,
    MotionController,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum InteractionModel {
    Touch,
    Crosshair,
    Classic,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum LabTableAction {
    Combine,
    React,
    Reset,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MultiPlayerSettingsAction {
    Enable,
    Disable,
    RefreshJoinCode,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ParamOption {
    None,
    CollapseEnum,
    HasSemanticConstraint,
    AsChainedCommand,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PhotoType {
    Portfolio,
    PhotoItem,
    Book,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ReleaseItemAction {
    Release,
    Consume,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SimpleEvent {
    None,
    CommandsEnabled,
    CommandsDisabled,
    UnlockWorldTemplateSettings,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum TeleportCause {
    None,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum UpdateBlockTransition {
    BlockToEntity,
    EntityToBlock,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum UseItemAction {
    ClickBlock,
    ClickAir,
    BreakBlock,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum UseItemMethod {
    EquipArmour,
    Eat,
    Attack,
    Consume,
    Throw,
    Shoot,
    Place,
    FillBottle,
    FillBucket,
    PourBucket,
    UseTool,
    Interact,
    Retrieved,
    Dyed,
    Traded,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum UseItemOnEntityAction {
    Interact,
    Attack,
}

