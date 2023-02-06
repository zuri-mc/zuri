use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use zuri_net_derive::packet;
use crate::proto::ints::VarI32;

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

#[packet(VarI32)]
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

#[derive(Debug, Clone)]
pub struct ContainerDataKey(pub i32);

impl Into<ContainerDataKey> for ContainerDataFurnace {
    fn into(self) -> ContainerDataKey {
        ContainerDataKey(self.to_i32().unwrap())
    }
}

impl Into<ContainerDataKey> for ContainerDataBrewingStand {
    fn into(self) -> ContainerDataKey {
        ContainerDataKey(self.to_i32().unwrap())
    }
}
