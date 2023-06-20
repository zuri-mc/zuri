use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

/// Provides the client a list of recipes that have been unlocked, restricting the recipes that
/// appear in the recipe book.
#[proto]
#[derive(Debug, Clone)]
pub struct UnlockedRecipes {
    /// The type of unlock that this packet will cause.
    pub unlock_type: UnlockedRecipesType,
    /// A list of recipe names that have been unlocked.
    #[len_type(VarU32)]
    pub recipes: Vec<String>,
}

/// Controls the type of unlock that a [UnlockedRecipes] will cause.
#[proto(u32)]
#[derive(Debug, Clone)]
pub enum UnlockedRecipesType {
    Empty,
    InitiallyUnlocked,
    NewlyUnlocked,
    RemoveUnlocked,
    RemoveAllUnlocked,
}
