use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum EntityDataType {
    Byte,
    I16,
    I32,
    F32,
    String,
    CompoundTag,
    BlockPos,
    I64,
    Vec3,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EntityDataKey {
    Flags,
    StructuralIntegrity,
    Variant,
    ColorIndex,
    Name,
    Owner,
    Target,
    AirSupply,
    EffectColor,
    EffectAmbience,
    JumpDuration,
    Hurt,
    HurtDirection,
    RowTimeLeft,
    RowTimeRight,
    Value,
    DisplayTileRuntimeID,
    DisplayOffset,
    CustomDisplay,
    Swell,
    OldSwell,
    SwellDirection,
    ChargeAmount,
    CarryBlockRuntimeID,
    ClientEvent,
    UsingItem,
    PlayerFlags,
    PlayerIndex,
    BedPosition,
    PowerX,
    PowerY,
    PowerZ,
    AuxPower,
    FishX,
    FishZ,
    FishAngle,
    AuxValueData,
    LeashHolder,
    Scale,
    HasNPC,
    NPCData,
    Actions,
    AirSupplyMax,
    MarkVariant,
    ContainerType,
    ContainerSize,
    ContainerStrengthModifier,
    BlockTarget,
    Inventory,
    TargetA,
    TargetB,
    TargetC,
    AerialAttack,
    Width,
    Height,
    FuseTime,
    SeatOffset,
    SeatLockPassengerRotation,
    SeatLockPassengerRotationDegrees,
    SeatRotationOffset,
    SeatRotationOffstDegrees,
    DataRadius,
    DataWaiting,
    DataParticle,
    PeekID,
    AttachFace,
    Attached,
    AttachedPosition,
    TradeTarget,
    Career,
    HasCommandBlock,
    CommandName,
    LastCommandOutput,
    TrackCommandOutput,
    ControllingSeatIndex,
    Strength,
    StrengthMax,
    DataSpellCastingColor,
    DataLifetimeTicks,
    PoseIndex,
    DataTickOffset,
    AlwaysShowNameTag,
    ColorTwoIndex,
    NameAuthor,
    Score,
    BalloonAnchor,
    PuffedState,
    BubbleTime,
    Agent,
    SittingAmount,
    SittingAmountPrevious,
    EatingCounter,
    FlagsTwo,
    LayingAmount,
    LayingAmountPrevious,
    DataDuration,
    DataSpawnTime,
    DataChangeRate,
    DataChangeOnPickup,
    DataPickupCount,
    InteractText,
    TradeTier,
    MaxTradeTier,
    TradeExperience,
    SkinID,
    SpawningFrames,
    CommandBlockTickDelay,
    CommandBlockExecuteOnFirstTick,
    AmbientSoundInterval,
    AmbientSoundIntervalRange,
    AmbientName,
    FallDamageMultiplier,
    NameRawText,
    CanRideTarget,
    LowTierCuredTradeDiscount,
    HighTierCuredTradeDiscount,
    NearbyCuredTradeDiscount,
    NearbyCuredDiscountTimeStamp,
    HitBox,
    IsBuoyant,
    FreezingEffectStrength,
    BuoyancyData,
    GoatHornCount,
    BaseRuntimeID,
    MovementSoundDistanceOffset,
    HeartbeatIntervalTicks,
    Heartbeat,
    PlayerLastDeathPosition,
    PlayerLastDeathDimension,
    PlayerHasDied,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EntityDataFlag {
    OnFire,
    Sneaking,
    Riding,
    Sprinting,
    UsingItem,
    Invisible,
    Tempted,
    InLove,
    Saddled,
    Powered,
    Ignited,
    Baby,
    Converting,
    Critical,
    ShowName,
    AlwaysShowName,
    NoAI,
    Silent,
    WallClimbing,
    Climb,
    Swim,
    Fly,
    Walk,
    Resting,
    Sitting,
    Angry,
    Interested,
    Charged,
    Tamed,
    Orphaned,
    Leashed,
    Sheared,
    Gliding,
    Elder,
    Moving,
    Breathing,
    Chested,
    Stackable,
    ShowBottom,
    Standing,
    Shaking,
    Idling,
    Casting,
    Charging,
    KeyboardControlled,
    PowerJump,
    Dash,
    Lingering,
    HasCollision,
    HasGravity,
    FireImmune,
    Dancing,
    Enchanted,
    ReturnTrident,
    ContainerPrivate,
    Transforming,
    DamageNearbyMobs,
    Swimming,
    Bribed,
    Pregnant,
    LayingEgg,
    PassengerCanPick,
    TransitionSitting,
    Eating,
    LayingDown,
    Sneezing,
    Trusting,
    Rolling,
    Scared,
    InScaffolding,
    OverScaffolding,
    DescendThroughBlock,
    Blocking,
    TransitionBlocking,
    BlockedUsingShield,
    BlockedUsingDamagedShield,
    Sleeping,
    WantsToWake,
    TradeInterest,
    DoorBreaker,
    BreakingObstruction,
    DoorOpener,
    Captain,
    Stunned,
    Roaring,
    DelayedAttack,
    AvoidingMobs,
    AvoidingBlock,
    FacingTargetToRangeAttack,
    HiddenWhenInvisible,
    InUI,
    Stalking,
    Emoting,
    Celebrating,
    Admiring,
    CelebratingSpecial,
    OutOfControl,
    RamAttack,
    PlayingDead,
    InAscendingBlock,
    OverDescendingBlock,
    Croaking,
    DigestMob,
    JumpGoal,
    Emerging,
    Sniffing,
    Digging,
    SonicBoom,
    HasDashTimeout,
    PushTowardsClosestSpace,
}