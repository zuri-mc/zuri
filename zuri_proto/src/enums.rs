use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Ability {
    Build,
    Mine,
    DoorsAndSwitches,
    OpenContainers,
    AttackPlayers,
    AttackMobs,
    OperatorCommands,
    Teleport,
    Invulnerable,
    Flying,
    MayFly,
    InstantBuild,
    Lightning,
    FlySpeed,
    WalkSpeed,
    Muted,
    WorldBuilder,
    NoClip,
    Count,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AbilityLayerType {
    CustomCache,
    Base,
    Spectator,
    Commands,
    Editor,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ActionPermission {
    Mine,
    DoorsAndSwitches,
    OpenContainers,
    AttackPlayers,
    AttackMobs,
    OperatorCommands,
    Teleport,
    Build,
    Default,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ActorEventType {
    Jump = 1,
    Hurt = 2,
    Death = 3,
    StartAttacking = 4,
    StopAttacking = 5,
    TamingFailed = 6,
    TamingSucceeded = 7,
    ShakeWetness = 8,
    UseItem = 9,
    EatGrass = 10,
    FishhookBubble = 11,
    FishhookFishPosition = 12,
    FishhookHookTime = 13,
    FishhookTease = 14,
    SquidFleeing = 15,
    ZombieConverting = 16,
    PlayAmbient = 17,
    SpawnAlive = 18,
    StartOfferFlower = 19,
    StopOfferFlower = 20,
    LoveHearts = 21,
    VillagerAngry = 22,
    VillagerHappy = 23,
    WitchHatMagic = 24,
    FireworksExplode = 25,
    InLoveHearts = 26,
    SilverfishMergeAnimation = 27,
    GuardianAttackSound = 28,
    DrinkPotion = 29,
    ThrowPotion = 30,
    CartWithPrimeTNT = 31,
    PrimeCreeper = 32,
    AirSupply = 33,
    AddPlayerLevels = 34,
    GuardianMiningFatigue = 35,
    AgentSwingArm = 36,
    DragonStartDeathAnim = 37,
    GroundDust = 38,
    Shake = 39,
    Feed = 57,
    BabyEat = 60,
    InstantDeath = 61,
    NotifyTrade = 62,
    LeashDestroyed = 63,
    CaravanUpdated = 64,
    TalismanActivate = 65,
    UpdateStructureFeature = 66,
    PlayerSpawnedMob = 67,
    Puke = 68,
    UpdateStackSize = 69,
    StartSwimming = 70,
    BalloonPop = 71,
    TreasureHunt = 72,
    SummonAgent = 73,
    FinishedChargingCrossbow = 74,
    LandedOnGround = 75,
    ActorGrowUp = 76,
    VibrationDetected = 77,
    DrinkMilk = 78,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AdventureFlag {
    WorldImmutable,
    NoPvM,
    NoPvP,
    Unused,
    ShowNameTags,
    AutoJump,
    AllowFlight,
    NoClip,
    WorldBuilder,
    Flying,
    Muted,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AgentActionType {
    None,
    Attack,
    Collect,
    Destroy,
    DetectRedstone,
    DetectObstacle,
    Drop,
    DropAll,
    Inspect,
    InspectData,
    InspectItemCount,
    InspectItemDetail,
    InspectItemSpace,
    Interact,
    Move,
    PlaceBlock,
    Till,
    TransferItemTo,
    Turn,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum AnimateAction {
    SwingArm = 1,
    StopSleep = 3,
    CriticalHit = 4,
    MagicCriticalHit = 5,
    RowRight = 128,
    RowLeft = 129,
}

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
pub enum BlockEventType {
    None,
    ChangeChestState,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum BlockUpdate {
    Neighbours,
    Network,
    NoGraphics,
    Priority,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BookAction {
    ReplacePage,
    AddPage,
    DeletePage,
    SwapPages,
    Sign,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BossEventType {
    Show,
    RegisterPlayer,
    Hide,
    UnregisterPlayer,
    HealthPercentage,
    Title,
    AppearanceProperties,
    Texture,
    Request,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum BossEventColour {
    Grey,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CameraShakeAction {
    Add,
    Stop,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CameraShakeType {
    Positional,
    Rotational,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ChatRestrictionLevel {
    None,
    Dropped,
    Disabled,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ClientBoundDebugRendererType {
    None,
    Clear,
    AddCube,
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CodeBuilderCategory {
    None,
    Status,
    Instantiation,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CodeBuilderOperation {
    None,
    Get,
    Set,
    Reset,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CommandArg {
    TypeInt = 1,
    TypeFloat = 3,
    TypeValue = 4,
    TypeWildcardInt = 5,
    TypeOperator = 6,
    TypeCompareOperator = 7,
    TypeTarget = 8,
    TypeWildcardTarget = 10,
    TypeFilepath = 17,
    TypeIntegerRange = 23,
    TypeEquipmentSlots = 38,
    TypeString = 39,
    TypeBlockPosition = 47,
    TypePosition = 48,
    TypeMessage = 51,
    TypeRawText = 53,
    TypeJSON = 57,
    TypeBlockStates = 67,
    TypeCommand = 70,

    Valid = 0x100000,
    Enum = 0x200000,
    Suffixed = 0x1000000,
    SoftEnum = 0x4000000,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CommandBlock {
    Impulse,
    Repeating,
    Chain,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CommandConstraint {
    CheatsEnabled,
    OperatorPermissions,
    HostPermissions,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CommandOriginType {
    Player,
    Block,
    MinecartBlock,
    DevConsole,
    Test,
    AutomationPlayer,
    ClientAutomation,
    DedicatedServer,
    Entity,
    Virtual,
    GameArgument,
    EntityServer,
    Precompiled,
    GameDirectorEntityServer,
    Script,
    Executor,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum CommandOutputType {
    None,
    LastOutput,
    Silent,
    AllOutput,
    DataSet,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CommandPermissionLevel {
    Normal,
    GameDirectors,
    Admin,
    Host,
    Owner,
    Internal,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum CompressionType {
    Flate,
    Snappy,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Container {
    AnvilInput,
    AnvilMaterial,
    AnvilResultPreview,
    SmithingTableInput,
    SmithingTableMaterial,
    SmithingTableResultPreview,
    Armor,
    LevelEntity,
    BeaconPayment,
    BrewingStandInput,
    BrewingStandResult,
    BrewingStandFuel,
    CombinedHotBarAndInventory,
    CraftingInput,
    CraftingOutputPreview,
    RecipeConstruction,
    RecipeNature,
    RecipeItems,
    RecipeSearch,
    RecipeSearchBar,
    RecipeEquipment,
    RecipeBook,
    EnchantingInput,
    EnchantingMaterial,
    FurnaceFuel,
    FurnaceIngredient,
    FurnaceResult,
    HorseEquip,
    HotBar,
    Inventory,
    ShulkerBox,
    TradeIngredientOne,
    TradeIngredientTwo,
    TradeResultPreview,
    Offhand,
    CompoundCreatorInput,
    CompoundCreatorOutputPreview,
    ElementConstructorOutputPreview,
    MaterialReducerInput,
    MaterialReducerOutput,
    LabTableInput,
    LoomInput,
    LoomDye,
    LoomMaterial,
    LoomResultPreview,
    BlastFurnaceIngredient,
    SmokerIngredient,
    TradeTwoIngredientOne,
    TradeTwoIngredientTwo,
    TradeTwoResultPreview,
    GrindstoneInput,
    GrindstoneAdditional,
    GrindstoneResultPreview,
    StonecutterInput,
    StonecutterResultPreview,
    CartographyInput,
    CartographyAdditional,
    CartographyResultPreview,
    Barrel,
    Cursor,
    CreatedOutput,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ContainerDataFurnace {
    TickCount = 0,
    LitTime = 1,
    LitDuration = 2,
    FuelAux = 4,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ContainerDataBrewingStand {
    BrewTime,
    FuelAmount,
    FuelTotal,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ContainerType {
    Inventory = -1,
    Container = 0,
    Workbench = 1,
    Furnace = 2,
    Enchantment = 3,
    BrewingStand = 4,
    Anvil = 5,
    Dispenser = 6,
    Dropper = 7,
    Hopper = 8,
    Cauldron = 9,
    CartChest = 10,
    CartHopper = 11,
    Horse = 12,
    Beacon = 13,
    StructureEditor = 14,
    Trade = 15,
    CommandBlock = 16,
    Jukebox = 17,
    Armour = 18,
    Hand = 19,
    CompoundCreator = 20,
    ElementConstructor = 21,
    MaterialReducer = 22,
    LabTable = 23,
    Loom = 24,
    Lectern = 25,
    Grindstone = 26,
    BlastFurnace = 27,
    Smoker = 28,
    Stonecutter = 29,
    Cartography = 30,
    HUD = 31,
    JigsawEditor = 32,
    SmithingTable = 33,
    ChestBoat = 34,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum EducationEditionRegion {
    None,
    RestOfWorld,
    China,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum EmoteFlag {
    ServerSide
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum EntityLinkType {
    Remove,
    Rider,
    Passenger,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum EventType {
    AchievementAwarded,
    EntityInteract,
    PortalBuilt,
    PortalUsed,
    MobKilled,
    CauldronUsed,
    PlayerDied,
    BossKilled,
    AgentCommand,
    AgentCreated,
    PatternRemoved,
    SlashCommandExecuted,
    FishBucketed,
    MobBorn,
    PetDied,
    CauldronInteract,
    ComposterInteract,
    BellUsed,
    EntityDefinitionTrigger,
    RaidUpdate,
    MovementAnomaly,
    MovementCorrected,
    ExtractHoney,
    TargetBlockHit,
    PiglinBarter,
    PlayerWaxedOrUnwaxedCopper,
    CodeBuilderRuntimeAction,
    CodeBuilderScoreboard,
    StriderRiddenInLavaInOverworld,
    SneakCloseToSculkSensor,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum FilterCause {
    ServerChatPublic,
    ServerChatWhisper,
    SignText,
    AnvilText,
    BookAndQuillText,
    CommandBlockText,
    BlockActorDataText,
    JoinEventText,
    LeaveEventText,
    SlashCommandChat,
    CartographyText,
    SlashCommandNonChat,
    ScoreboardText,
    TickingAreaText,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum GamePublishSetting {
    None,
    InviteOnly,
    FriendsOnly,
    FriendsOfFriends,
    Public,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum GameTestRequestRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    Rotate360,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    Default,
    Spectator,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Generator {
    Legacy,
    Overworld,
    Flat,
    Nether,
    End,
    Void,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum HeightMapType {
    None,
    HasData,
    TooHigh,
    TooLow,
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

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum InteractionAction {
    LeaveVehicle = 3,
    MouseOverEntity = 4,
    NPCOpen = 5,
    OpenInventory = 6,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum InteractionModel {
    Touch,
    Crosshair,
    Classic,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum InventoryActionSource {
    Container = 0,
    World = 2,
    Creative = 3,
    TODO = 99999,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum InventoryTransactionType {
    Normal,
    Mismatch,
    UseItem,
    UseItemOnEntity,
    ReleaseItem,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ItemDescriptorType {
    Invalid,
    Default,
    MoLang,
    ItemTag,
    Deferred,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ItemStackResponseStatus {
    Ok,
    Error,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum LabTableAction {
    Combine,
    React,
    Reset,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum LessonAction {
    Start,
    Complete,
    Restart,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum LevelEventType {
    SoundClick = 1000,
    SoundClickFail = 1001,
    SoundLaunch = 1002,
    SoundOpenDoor = 1003,
    SoundFizz = 1004,
    SoundFuse = 1005,
    SoundPlayRecording = 1006,
    SoundGhastWarning = 1007,
    SoundGhastFireball = 1008,
    SoundBlazeFireball = 1009,
    SoundZombieWoodenDoor = 1010,
    SoundZombieDoorCrash = 1012,
    SoundZombieInfected = 1016,
    SoundZombieConverted = 1017,
    SoundEndermanTeleport = 1018,
    SoundAnvilBroken = 1020,
    SoundAnvilUsed = 1021,
    SoundAnvilLand = 1022,
    SoundInfinityArrowPickup = 1030,
    SoundTeleportEnderPearl = 1032,
    SoundAddItem = 1040,
    SoundItemFrameBreak = 1041,
    SoundItemFramePlace = 1042,
    SoundItemFrameRemoveItem = 1043,
    SoundItemFrameRotateItem = 1044,
    SoundExperienceOrbPickup = 1051,
    SoundTotemUsed = 1052,
    SoundArmorStandBreak = 1060,
    SoundArmorStandHit = 1061,
    SoundArmorStandLand = 1062,
    SoundArmorStandPlace = 1063,
    SoundPointedDripstoneLand = 1064,
    SoundDyeUsed = 1065,
    SoundInkSacUsed = 1066,
    QueueCustomMusic = 1900,
    PlayCustomMusic = 1901,
    StopCustomMusic = 1902,
    SetMusicVolume = 1903,
    ParticlesShoot = 2000,
    ParticlesDestroyBlock = 2001,
    ParticlesPotionSplash = 2002,
    ParticlesEyeOfEnderDeath = 2003,
    ParticlesMobBlockSpawn = 2004,
    ParticleCropGrowth = 2005,
    ParticleSoundGuardianGhost = 2006,
    ParticleDeathSmoke = 2007,
    ParticleDenyBlock = 2008,
    ParticleGenericSpawn = 2009,
    ParticlesDragonEgg = 2010,
    ParticlesCropEaten = 2011,
    ParticlesCritical = 2012,
    ParticlesTeleport = 2013,
    ParticlesCrackBlock = 2014,
    ParticlesBubble = 2015,
    ParticlesEvaporate = 2016,
    ParticlesDestroyArmorStand = 2017,
    ParticlesBreakingEgg = 2018,
    ParticleDestroyEgg = 2019,
    ParticlesEvaporateWater = 2020,
    ParticlesDestroyBlockNoSound = 2021,
    ParticlesKnockbackRoar = 2022,
    ParticlesTeleportTrail = 2023,
    ParticlesPointCloud = 2024,
    ParticlesExplosion = 2025,
    ParticlesBlockExplosion = 2026,
    ParticlesVibrationSignal = 2027,
    ParticlesDripstoneDrip = 2028,
    ParticlesFizzEffect = 2029,
    WaxOn = 2030,
    WaxOff = 2031,
    Scrape = 2032,
    ParticlesElectricSpark = 2033,
    ParticleTurtleEgg = 2034,
    ParticleSculkShriek = 2035,
    SculkCatalystBloom = 2036,
    SculkCharge = 2037,
    SculkChargePop = 2038,
    SonicExplosion = 2039,
    StartRaining = 3001,
    StartThunderstorm = 3002,
    StopRaining = 3003,
    StopThunderstorm = 3004,
    GlobalPause = 3005,
    SimTimeStep = 3006,
    SimTimeScale = 3007,
    ActivateBlock = 3500,
    CauldronExplode = 3501,
    CauldronDyeArmor = 3502,
    CauldronCleanArmor = 3503,
    CauldronFillPotion = 3504,
    CauldronTakePotion = 3505,
    CauldronFillWater = 3506,
    CauldronTakeWater = 3507,
    CauldronAddDye = 3508,
    CauldronCleanBanner = 3509,
    CauldronFlush = 3510,
    AgentSpawnEffect = 3511,
    CauldronFillLava = 3512,
    CauldronTakeLava = 3513,
    CauldronFillPowderSnow = 3514,
    CauldronTakePowderSnow = 3515,
    StartBlockCracking = 3600,
    StopBlockCracking = 3601,
    UpdateBlockCracking = 3602,
    AllPlayersSleeping = 9800,
    SleepingPlayers = 9801,
    JumpPrevented = 9810,
    ParticleLegacyEvent = 0x4000,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MapObjectType {
    Entity,
    Block,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum MapUpdateFlag {
    Texture,
    Decoration,
    Initialisation,
}

impl MapUpdateFlag {
    pub fn flag(&self) -> u32 {
        1 << (*self as u32)
    }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MobEffectType {
    Speed,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneation,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Levitation,
    FatalPoison,
    ConduitPower,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MobEffectOperation {
    Add,
    Modify,
    Remove,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ModalFormCancelReason {
    UserClosed,
    UserBusy,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum MoveActorDeltaFlag {
    HasX,
    HasY,
    HasZ,
    HasRotX,
    HasRotY,
    HasRotZ,
    OnGround,
    Teleport,
    ForceMove,
}

impl MoveActorDeltaFlag {
    pub fn flag(&self) -> u16 {
        1 << (*self as u16)
    }
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MoveFlag {
    OnGround,
    Teleport,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MoveMode {
    Normal,
    Reset,
    Teleport,
    Rotation,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MultiPlayerSettingsAction {
    Enable,
    Disable,
    RefreshJoinCode,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum NPCDialogueAction {
    Open,
    Close,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum NPCRequestAction {
    SetActions,
    ExecuteAction,
    ExecuteClosingCommands,
    SetName,
    SetSkin,
    SetInteractText,
    ExecuteOpeningCommands,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PacketViolationSeverity {
    Warning,
    FinalWarning,
    TerminatingConnection,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PacketViolationType {
    Malformed,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ParamOption {
    None,
    CollapseEnum,
    HasSemanticConstraint,
    AsChainedCommand,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PhotoType {
    Portfolio,
    PhotoItem,
    Book,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
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

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PlayerListAction {
    Add,
    Remove,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PlayerMovementMode {
    Client,
    Server,
    ServerWithRewind,
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PlayStatusType {
    LoginSuccess,
    LoginFailedClient,
    LoginFailedServer,
    PlayerSpawn,
    LoginFailedInvalidTenant,
    LoginFailedVanillaEdu,
    LoginFailedEduVanilla,
    LoginFailedServerFull,
    LoginFailedEditorVanilla,
    LoginFailedVanillaEditor,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PositionTrackingDBBroadcastAction {
    Update,
    Destroy,
    NotFound,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PositionTrackingDBRequestAction {
    Query
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum RecipeType {
    Shapeless,
    Shaped,
    Furnace,
    FurnaceData,
    Multi,
    ShulkerBox,
    ShapelessChemistry,
    ShapedChemistry,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ReleaseItemAction {
    Release,
    Consume,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ResourcePackResponse {
    None,
    Refused,
    SendPacks,
    AllPacksDownloaded,
    Completed,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ResourcePackType {
    Addon,
    Cached,
    CopyProtected,
    Behaviour,
    PersonaPiece,
    Resources,
    Skins,
    WorldTemplate,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum RespawnState {
    SearchingForSpawn,
    ReadyToSpawn,
    ClientReadyToSpawn,
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ScoreboardAction {
    Modify,
    Remove,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ScoreboardIdentity {
    Player,
    Entity,
    FakePlayer,
}

#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ScoreboardIdentityAction {
    Register,
    Clear,
}

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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ScoreboardSortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum ShowCreditsStatus {
    Start,
    End,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SimpleEvent {
    None,
    CommandsEnabled,
    CommandsDisabled,
    UnlockWorldTemplateSettings,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Simulation {
    Game,
    Editor,
    Test,
    Invalid,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SoftEnumAction {
    Add,
    Remove,
    Set,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SoundEvent {
    ItemUseOn,
    Hit,
    Step,
    Fly,
    Jump,
    Break,
    Place,
    HeavyStep,
    Gallop,
    Fall,
    Ambient,
    AmbientBaby,
    AmbientInWater,
    Breathe,
    Death,
    DeathInWater,
    DeathToZombie,
    Hurt,
    HurtInWater,
    Mad,
    Boost,
    Bow,
    SquishBig,
    SquishSmall,
    FallBig,
    FallSmall,
    Splash,
    Fizz,
    Flap,
    Swim,
    Drink,
    Eat,
    Takeoff,
    Shake,
    Plop,
    Land,
    Saddle,
    Armor,
    ArmorPlace,
    AddChest,
    Throw,
    Attack,
    AttackNoDamage,
    AttackStrong,
    Warn,
    Shear,
    Milk,
    Thunder,
    Explode,
    Fire,
    Ignite,
    Fuse,
    Stare,
    Spawn,
    Shoot,
    BreakBlock,
    Launch,
    Blast,
    LargeBlast,
    Twinkle,
    Remedy,
    Unfect,
    LevelUp,
    BowHit,
    BulletHit,
    ExtinguishFire,
    ItemFizz,
    ChestOpen,
    ChestClosed,
    ShulkerBoxOpen,
    ShulkerBoxClosed,
    EnderChestOpen,
    EnderChestClosed,
    PowerOn,
    PowerOff,
    Attach,
    Detach,
    Deny,
    Tripod,
    Pop,
    DropSlot,
    Note,
    Thorns,
    PistonIn,
    PistonOut,
    Portal,
    Water,
    LavaPop,
    Lava,
    Burp,
    BucketFillWater,
    BucketFillLava,
    BucketEmptyWater,
    BucketEmptyLava,
    EquipChain,
    EquipDiamond,
    EquipGeneric,
    EquipGold,
    EquipIron,
    EquipLeather,
    EquipElytra,
    Record13,
    RecordCat,
    RecordBlocks,
    RecordChirp,
    RecordFar,
    RecordMall,
    RecordMellohi,
    RecordStal,
    RecordStrad,
    RecordWard,
    Record11,
    RecordWait,
    RecordNull,
    Flop,
    GuardianCurse,
    MobWarning,
    MobWarningBaby,
    Teleport,
    ShulkerOpen,
    ShulkerClose,
    Haggle,
    HaggleYes,
    HaggleNo,
    HaggleIdle,
    ChorusGrow,
    ChorusDeath,
    Glass,
    PotionBrewed,
    CastSpell,
    PrepareAttackSpell,
    PrepareSummon,
    PrepareWololo,
    Fang,
    Charge,
    TakePicture,
    PlaceLeashKnot,
    BreakLeashKnot,
    AmbientGrowl,
    AmbientWhine,
    AmbientPant,
    AmbientPurr,
    AmbientPurreow,
    DeathMinVolume,
    DeathMidVolume,
    ImitateBlaze,
    ImitateCaveSpider,
    ImitateCreeper,
    ImitateElderGuardian,
    ImitateEnderDragon,
    ImitateEnderman,
    ImitateEndermite,
    ImitateEvocationIllager,
    ImitateGhast,
    ImitateHusk,
    ImitateIllusionIllager,
    ImitateMagmaCube,
    ImitatePolarBear,
    ImitateShulker,
    ImitateSilverfish,
    ImitateSkeleton,
    ImitateSlime,
    ImitateSpider,
    ImitateStray,
    ImitateVex,
    ImitateVindicationIllager,
    ImitateWitch,
    ImitateWither,
    ImitateWitherSkeleton,
    ImitateWolf,
    ImitateZombie,
    ImitateZombiePigman,
    ImitateZombieVillager,
    EnderEyePlaced,
    EndPortalCreated,
    AnvilUse,
    BottleDragonBreath,
    PortalTravel,
    TridentHit,
    TridentReturn,
    TridentRiptide1,
    TridentRiptide2,
    TridentRiptide3,
    TridentThrow,
    TridentThunder,
    TridentHitGround,
    Default,
    FletchingTableUse,
    ElemConstructOpen,
    IceBombHit,
    BalloonPop,
    LtReactionIceBomb,
    LtReactionBleach,
    LtReactionElephantToothpaste,
    LtReactionElephantToothpaste2,
    LtReactionGlowStick,
    LtReactionGlowStick2,
    LtReactionLuminol,
    LtReactionSalt,
    LtReactionFertilizer,
    LtReactionFireball,
    LtReactionMagnesiumSalt,
    LtReactionMiscFire,
    LtReactionFire,
    LtReactionMiscExplosion,
    LtReactionMiscMystical,
    LtReactionMiscMystical2,
    LtReactionProduct,
    SparklerUse,
    GlowStickUse,
    SparklerActive,
    ConvertToDrowned,
    BucketFillFish,
    BucketEmptyFish,
    BubbleColumnUpwards,
    BubbleColumnDownwards,
    BubblePop,
    BubbleUpInside,
    BubbleDownInside,
    HurtBaby,
    DeathBaby,
    StepBaby,
    SpawnBaby,
    Born,
    TurtleEggBreak,
    TurtleEggCrack,
    TurtleEggHatched,
    LayEgg,
    TurtleEggAttacked,
    BeaconActivate,
    BeaconAmbient,
    BeaconDeactivate,
    BeaconPower,
    ConduitActivate,
    ConduitAmbient,
    ConduitAttack,
    ConduitDeactivate,
    ConduitShort,
    Swoop,
    BambooSaplingPlace,
    PreSneeze,
    Sneeze,
    AmbientTame,
    Scared,
    ScaffoldingClimb,
    CrossbowLoadingStart,
    CrossbowLoadingMiddle,
    CrossbowLoadingEnd,
    CrossbowShoot,
    CrossbowQuickChargeStart,
    CrossbowQuickChargeMiddle,
    CrossbowQuickChargeEnd,
    AmbientAggressive,
    AmbientWorried,
    CantBreed,
    ShieldBlock,
    LecternBookPlace,
    GrindstoneUse,
    Bell,
    CampfireCrackle,
    Roar,
    Stun,
    SweetBerryBushHurt,
    SweetBerryBushPick,
    CartographyTableUse,
    StonecutterUse,
    ComposterEmpty,
    ComposterFill,
    ComposterFillLayer,
    ComposterReady,
    BarrelOpen,
    BarrelClose,
    RaidHorn,
    LoomUse,
    AmbientInRaid,
    UicartographyTableUse,
    UistonecutterUse,
    UiloomUse,
    SmokerUse,
    BlastFurnaceUse,
    SmithingTableUse,
    Screech,
    Sleep,
    FurnaceUse,
    MooshroomConvert,
    MilkSuspiciously,
    Celebrate,
    JumpPrevent,
    AmbientPollinate,
    BeehiveDrip,
    BeehiveEnter,
    BeehiveExit,
    BeehiveWork,
    BeehiveShear,
    HoneybottleDrink,
    AmbientCave,
    Retreat,
    ConvertToZombified,
    Admire,
    StepLava,
    Tempt,
    Panic,
    Angry,
    AmbientMoodWarpedForest,
    AmbientMoodSoulsandValley,
    AmbientMoodNetherWastes,
    AmbientMoodBasaltDeltas,
    AmbientMoodCrimsonForest,
    RespawnAnchorCharge,
    RespawnAnchorDeplete,
    RespawnAnchorSetSpawn,
    RespawnAnchorAmbient,
    SoulEscapeQuiet,
    SoulEscapeLoud,
    RecordPigstep,
    LinkCompassToLodestone,
    UseSmithingTable,
    EquipNetherite,
    AmbientLoopWarpedForest,
    AmbientLoopSoulsandValley,
    AmbientLoopNetherWastes,
    AmbientLoopBasaltDeltas,
    AmbientLoopCrimsonForest,
    AmbientAdditionWarpedForest,
    AmbientAdditionSoulsandValley,
    AmbientAdditionNetherWastes,
    AmbientAdditionBasaltDeltas,
    AmbientAdditionCrimsonForest,
    SculkSensorPowerOn,
    SculkSensorPowerOff,
    BucketFillPowderSnow,
    BucketEmptyPowderSnow,
    PointedDripstoneCauldronDripWater,
    PointedDripstoneCauldronDripLava,
    PointedDripstoneDripWater,
    PointedDripstoneDripLava,
    CaveVinesPickBerries,
    BigDripleafTiltDown,
    BigDripleafTiltUp,
    CopperWaxOn,
    CopperWaxOff,
    Scrape,
    PlayerHurtDrown,
    PlayerHurtOnFire,
    PlayerHurtFreeze,
    UseSpyglass,
    StopUsingSpyglass,
    AmethystBlockChime,
    AmbientScreamer,
    HurtScreamer,
    DeathScreamer,
    MilkScreamer,
    JumpToBlock,
    PreRam,
    PreRamScreamer,
    RamImpact,
    RamImpactScreamer,
    SquidInkSquirt,
    GlowSquidInkSquirt,
    ConvertToStray,
    CakeAddCandle,
    ExtinguishCandle,
    AmbientCandle,
    BlockClick,
    BlockClickFail,
    SculkCatalystBloom,
    SculkShriekerShriek,
    WardenNearbyClose,
    WardenNearbyCloser,
    WardenNearbyClosest,
    WardenSlightlyAngry,
    RecordOtherside,
    Tongue,
    CrackIronGolem,
    RepairIronGolem,
    Listening,
    Heartbeat,
    HornBreak,
    SculkSpread = 379,
    SculkCharge = 380,
    SculkSensorPlace = 381,
    SculkShriekerPlace = 382,
    GoatCall0 = 383,
    GoatCall1 = 384,
    GoatCall2 = 385,
    GoatCall3 = 386,
    GoatCall4 = 387,
    GoatCall5 = 388,
    GoatCall6 = 389,
    GoatCall7 = 390,
    ImitateWarden = 426,
    ListeningAngry = 427,
    ItemGiven = 428,
    ItemTaken = 429,
    Disappeared = 430,
    Reappeared = 431,
    DrinkMilk = 432,
    FrogspawnHatched = 433,
    LaySpawn = 434,
    FrogspawnBreak = 435,
    SonicBoom = 436,
    SonicCharge = 437,
    ItemThrown = 438,
    Record5 = 439,
    ConvertToFrog = 440,
    RecordPlaying = 441,
    EnchantingTableUse = 442,
    StepSand = 443,
    DashReady = 444,
    BundleDropContents = 445,
    BundleInsert = 446,
    BundleRemoveOne = 447,
    PressurePlateClickOff = 448,
    PressurePlateClickOn = 449,
    ButtonClickOff = 450,
    ButtonClickOn = 451,
    DoorOpen = 452,
    DoorClose = 453,
    TrapdoorOpen = 454,
    TrapdoorClose = 455,
    FenceGateOpen = 456,
    FenceGateClose = 457,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SpawnBiomeType {
    Default,
    USerDefined,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SpawnType {
    Player,
    World,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StackRequestActionType {
    Take,
    Place,
    Swap,
    Drop,
    Destroy,
    Consume,
    Create,
    PlaceInContainer,
    TakeOutContainer,
    LabTableCombine,
    BeaconPayment,
    MineBlock,
    CraftRecipe,
    CraftRecipeAuto,
    CraftCreative,
    CraftRecipeOptional,
    CraftGrindstone,
    CraftLoom,
    CraftNonImplementedDeprecated,
    CraftResultsDeprecated,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureBlockType {
    Data,
    Save,
    Load,
    Corner,
    Invalid,
    Export,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureMirrorAxis {
    None,
    X,
    Z,
    Both,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureRedstoneSaveMode {
    Memory,
    Disk,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureRotation {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureTemplateDataRequestType {
    None,
    ExportFromSave,
    ExportFromLoad,
    QuerySavedStructure,
    ImportFromSave,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum StructureTemplateDataResponseType {
    Export,
    Query,
    Import,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum SubChunkRequestMode {
    Legacy,
    Limitless,
    Limited,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SubChunkResult {
    Success,
    ChunkNotFound,
    InvalidDimension,
    PlayerNotFound,
    IndexOutOfBounds,
    SuccessAllAir,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum TeleportCause {
    None,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TextType {
    Raw,
    Chat,
    Translation,
    Popup,
    JukeboxPopup,
    Tip,
    System,
    Whisper,
    Announcement,
    ObjectWhisper,
    Object,
    ObjectAnnouncement,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum TitleAction {
    Clear,
    Reset,
    SetTitle,
    SetSubtitle,
    SetActionBar,
    SetDurations,
    TitleTextObject,
    SubtitleTextObject,
    ActionbarTextObject,
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

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Window {
    Inventory = 0,
    OffHand = 119,
    Armour = 120,
    UI = 124,
}
