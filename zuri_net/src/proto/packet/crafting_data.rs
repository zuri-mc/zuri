use crate::proto::ints::VarU32;
use crate::proto::types::recipe::{
    MaterialReducer, PotionContainerChangeRecipe, PotionRecipe, Recipe,
};
use zuri_net_derive::proto;

/// Sent by the server to let the client know all crafting data that the server maintains. This
/// includes shapeless crafting, crafting table recipes, furnace recipes etc. Each crafting
/// station's recipes are included in it.
#[proto]
#[derive(Debug, Clone)]
pub struct CraftingData {
    /// List of all recipes available on the server. It includes among others shapeless, shaped and
    /// furnace recipes. The client will only be able to craft these recipes.
    #[len_type(VarU32)]
    pub recipes: Vec<Recipe>,
    /// List of all potion mixing recipes which may be used in the brewing stand.
    #[len_type(VarU32)]
    pub potion_recipes: Vec<PotionRecipe>,
    /// List of all recipes to convert a potion from one type to another, such as from a drinkable
    /// potion to a splash potion, or from a splash potion to a lingering potion.
    #[len_type(VarU32)]
    pub potion_container_change_recipes: Vec<PotionContainerChangeRecipe>,
    /// List of all material reducers. These are primarily used in the Education Edition chemistry
    /// system.
    #[len_type(VarU32)]
    pub material_reducers: Vec<MaterialReducer>,
    /// Indicates if all recipes currently active on the client should be cleaned. Doing this means
    /// that the client will have no recipes active by itself: any CraftingData packets previously
    /// sent will also be discarded, and only the recipes in this CraftingData packet will be used.
    pub clear_recipes: bool,
}
