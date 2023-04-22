use zuri_net_derive::proto;
use crate::proto::ints::VarU32;

/// Provides the client a list of recipes that have been unlocked, restricting the recipes that
/// appear in the recipe book.
#[proto]
#[derive(Debug, Clone)]
pub struct UnlockedRecipes {
    /// Determines if new recipes have been unlocked since the packet was last sent.
    pub new_unlocks: bool,
    /// A list of recipe names that have been unlocked.
    #[len_type(VarU32)]
    pub recipes: Vec<String>,
}
