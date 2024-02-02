use zuri_net_derive::proto;

#[proto]
#[derive(Debug, PartialEq, Clone)]
pub struct SetPlayerInventoryOptions {
    pub left_inventory_tab: InventoryLeftTab,
    pub right_inventory_tab: InventoryRightTab,
    pub filtering: bool,
    pub inventory_layout: InventoryLayout,
    pub crafting_layout: InventoryLayout,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryLayout {
    None,
    Survival,
    RecipeBook,
    Creative,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryLeftTab {
    None,
    Construction,
    Equipment,
    Items,
    Nature,
    Search,
    Survival,
}

#[proto(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryRightTab {
    None,
    FullScreen,
    Crafting,
    Armour,
}
