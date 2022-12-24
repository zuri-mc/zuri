/// Sent by the server to let the client know all crafting data that the server maintains. This includes shapeless
/// crafting, crafting table recipes, furnace recipes etc. Each crafting station's recipes are included in it.
#[derive(Debug)]
pub struct CraftingData {
    /// List of all recipes available on the server. It includes among others shapeless, shaped and furnace recipes. The
    /// client will only be able to craft these recipes.
    pub recipes: Vec<RecipeType>,
    // TODO: Recipe trait
    /// List of all potion mixing recipes which may be used in the brewing stand.
    pub potion_recipes: Vec<PotionRecipe>,
    /// List of all recipes to convert a potion from one type to another, such as from a drinkable potion to a splash
    /// potion, or from a splash potion to a lingering potion.
    pub potion_container_change_recipes: Vec<PotionContainerChangeRecipe>,
    /// List of all material reducers. These are primarily used in the Education Edition chemistry system.
    pub material_reducers: Vec<MaterialReducer>,
    /// Indicates if all recipes currently active on the client should be cleaned. Doing this means that the client will
    /// have no recipes active by itself: any CraftingData packets previously sent will also be discarded, and only the
    /// recipes in this CraftingData packet will be used.
    pub clear_recipes: bool,
}

impl Packet for CraftingData {
    fn write(&self, writer: &mut Writer) {
        todo!()
        // writer.write_TODO(self.LEN);
        // writer.write_Recipe(self.recipes);
        // writer.write_TODO(self.LEN);
        // writer.write_PotionRecipe(self.potion_recipes);
        // writer.write_TODO(self.LEN);
        // writer.write_PotionContainerChangeRecipe(self.potion_container_change_recipes);
        // writer.write_TODO(self.LEN);
        // writer.write_MaterialReducer(self.material_reducers);
        // writer.bool(self.clear_recipes);
    }

    fn read(reader: &mut Reader) -> Self {
        todo!()
        // Self {
        //     LEN: reader.read_TODO(),
        //     recipes: reader.read_Recipe(),
        //     LEN: reader.read_TODO(),
        //     potion_recipes: reader.read_PotionRecipe(),
        //     LEN: reader.read_TODO(),
        //     potion_container_change_recipes: reader.read_PotionContainerChangeRecipe(),
        //     LEN: reader.read_TODO(),
        //     material_reducers: reader.read_MaterialReducer(),
        //     clear_recipes: reader.bool(),
        // };
    }
}
