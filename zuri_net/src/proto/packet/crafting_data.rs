use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::recipe::{MaterialReducer, PotionContainerChangeRecipe, PotionRecipe, Recipe};

/// Sent by the server to let the client know all crafting data that the server maintains. This
/// includes shapeless crafting, crafting table recipes, furnace recipes etc. Each crafting
/// station's recipes are included in it.
#[derive(Debug, Clone)]
pub struct CraftingData {
    /// List of all recipes available on the server. It includes among others shapeless, shaped and
    /// furnace recipes. The client will only be able to craft these recipes.
    pub recipes: Vec<Recipe>,
    // TODO: Recipe trait
    /// List of all potion mixing recipes which may be used in the brewing stand.
    pub potion_recipes: Vec<PotionRecipe>,
    /// List of all recipes to convert a potion from one type to another, such as from a drinkable
    /// potion to a splash potion, or from a splash potion to a lingering potion.
    pub potion_container_change_recipes: Vec<PotionContainerChangeRecipe>,
    /// List of all material reducers. These are primarily used in the Education Edition chemistry
    /// system.
    pub material_reducers: Vec<MaterialReducer>,
    /// Indicates if all recipes currently active on the client should be cleaned. Doing this means
    /// that the client will have no recipes active by itself: any CraftingData packets previously
    /// sent will also be discarded, and only the recipes in this CraftingData packet will be used.
    pub clear_recipes: bool,
}

impl PacketType for CraftingData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.recipes.len() as u32);
        self.recipes.iter().for_each(|recipe| recipe.write(writer));
        self.potion_recipes.iter().for_each(|recipe| recipe.write(writer));
        self.potion_container_change_recipes.iter().for_each(|recipe| recipe.write(writer));
        self.material_reducers.iter().for_each(|reducer| reducer.write(writer));
        writer.bool(self.clear_recipes);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            recipes: (0..reader.var_u32())
                .map(|_| Recipe::read(reader))
                .collect(),
            potion_recipes: (0..reader.var_u32())
                .map(|_| PotionRecipe::read(reader))
                .collect(),
            potion_container_change_recipes: (0..reader.var_u32())
                .map(|_| PotionContainerChangeRecipe::read(reader))
                .collect(),
            material_reducers: (0..reader.var_u32())
                .map(|_| MaterialReducer::read(reader))
                .collect(),
            clear_recipes: reader.bool(),
        }
    }
}
