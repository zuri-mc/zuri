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

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum BookAction {
    ReplacePage,
    AddPage,
    DeletePage,
    SwapPages,
    Sign,
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum CommandOutputType {
    None,
    LastOutput,
    Silent,
    AllOutput,
    DataSet,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum CommandPermissionLevel {
    Normal,
    GameDirectors,
    Admin,
    Host,
    Owner,
    Internal,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum CompressionType {
    Flate,
    Snappy,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum ContainerDataFurnace {
    TickCount = 0,
    LitTime = 1,
    LitDuration = 2,
    FuelAux = 4,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum ContainerDataBrewingStand {
    BrewTime,
    FuelAmount,
    FuelTotal,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EducationEditionRegion {
    None,
    RestOfWorld,
    China,
}

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum EmoteFlag {
    ServerSide
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

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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
}s

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
pub enum ReleaseItemAction {
    Release,
    Consume,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum RespawnState {
    SearchingForSpawn,
    ReadyToSpawn,
    ClientReadyToSpawn,
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

#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
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
