use uuid::Uuid;
use zuri_net_derive::proto;

use crate::proto::ints::{VarI32, VarU32};
use crate::proto::io::{Readable, Reader, Writable, Writer};
use crate::proto::types::item::ItemStack;
use crate::proto::types::item_descriptor::ItemDescriptorCount;

#[proto(VarU32)]
#[repr(u32)]
#[derive(Debug, Clone)]
pub enum Recipe {
    ShapelessRecipe(ShapelessRecipe),
    ShapedRecipe(ShapedRecipe),
    FurnaceRecipe(FurnaceRecipe),
    FurnaceDataRecipe(FurnaceRecipe),
    MultiRecipe(MultiRecipe),
    ShulkerBoxRecipe(ShulkerBoxRecipe),
    ShapelessChemistryRecipe(ShapelessChemistryRecipe),
    ShapedChemistryRecipe(ShapedChemistryRecipe),
    SmithingTransform(SmithingTransformRecipe),
    SmithingTrim(SmithingTrimRecipe),
}

/// A recipe specifically used for smithing tables. It has two input items and adds them together,
/// resulting in a new item.
#[proto]
#[derive(Debug, Clone)]
pub struct SmithingTransformRecipe {
    /// A unique ID used to identify the recipe over network. Each recipe must have a unique network
    /// ID. Recommended is to just increment a variable for each unique recipe registered. This
    /// field must never be 0.
    pub recipe_network_id: u32,
    /// A unique ID of the recipe. This ID must be unique amongst all other types of recipes too,
    /// but its functionality is not exactly known.
    pub recipe_id: String,
    /// The item that is used to shape the Base item based on the Addition being applied.
    pub template: ItemDescriptorCount,
    /// The item that the Addition is being applied to in the smithing table.
    pub base: ItemDescriptorCount,
    /// The item that is being added to the Base item to result in a modified item.
    pub addition: ItemDescriptorCount,
    /// The resulting item from the two items being added together.
    pub result: ItemStack,
    /// The block name that is required to create the output of the recipe. The block is not
    /// prefixed with 'minecraft:', so it will look like 'smithing_table' as an example.
    pub block: String,
}

pub type SmithingTrimRecipe = ShapelessRecipe;

#[proto]
#[derive(Debug, Clone)]
pub struct MultiRecipe {
    pub uuid: Uuid,
    pub recipe_network_id: VarU32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct FurnaceDataRecipe {
    pub furnace_recipe: FurnaceRecipe,
}

#[proto]
#[derive(Debug, Clone)]
pub struct FurnaceRecipe {
    pub network_id: VarI32,
    pub output: ItemStack,
    pub block: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PotionRecipe {
    pub input_potion_id: VarI32,
    pub input_potion_metadata: VarI32,
    pub reagent_item_id: VarI32,
    pub reagent_item_metadata: VarI32,
    pub output_potion_id: VarI32,
    pub output_potion_metadata: VarI32,
}

pub type ShapedChemistryRecipe = ShapedRecipe;

#[derive(Debug, Clone, Default)]
pub struct ShapedRecipe {
    pub recipe_id: String,
    pub width: i32,
    pub height: i32,
    pub input: Vec<ItemDescriptorCount>,
    pub output: Vec<ItemStack>,
    pub uuid: Uuid,
    pub block: String,
    pub priority: i32,
    pub recipe_network_id: u32,
}

impl Writable for ShapedRecipe {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.recipe_id.as_str());
        writer.i32(self.width);
        writer.i32(self.height);
        for i in 0..self.width * self.height {
            if i >= self.input.len() as i32 {
                ItemDescriptorCount::default().write(writer);
            } else {
                self.input[i as usize].write(writer);
            }
        }
        writer.var_u32(self.output.len() as u32);
        self.output.iter().for_each(|stack| stack.write(writer));
        writer.uuid(self.uuid);
        writer.string(self.block.as_str());
        writer.var_i32(self.priority);
        writer.var_u32(self.recipe_network_id);
    }
}

impl Readable<ShapedRecipe> for ShapedRecipe {
    fn read(reader: &mut Reader) -> Self {
        let recipe_id = reader.string();
        let width = reader.i32();
        let height = reader.i32();
        Self {
            recipe_id,
            width,
            height,
            input: (0..width * height)
                .map(|_| ItemDescriptorCount::read(reader))
                .collect(),
            output: (0..reader.var_u32())
                .map(|_| ItemStack::read(reader))
                .collect(),
            uuid: reader.uuid(),
            block: reader.string(),
            priority: reader.var_i32(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

pub type ShapelessChemistryRecipe = ShapelessRecipe;

#[proto]
#[derive(Debug, Clone)]
pub struct ShapelessRecipe {
    pub recipe_id: String,
    #[len_type(VarU32)]
    pub input: Vec<ItemDescriptorCount>,
    #[len_type(VarU32)]
    pub output: Vec<ItemStack>,
    pub uuid: Uuid,
    pub block: String,
    pub priority: VarI32,
    pub recipe_network_id: VarU32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct MaterialReducer {
    pub network_id: i32,
    pub metadata_value: u32,
    #[len_type(VarU32)]
    pub outputs: Vec<MaterialReducerOutput>,
}

#[derive(Debug, Clone)]
pub struct ItemType {
    pub network_id: i32,
    pub metadata_value: u32,
}

impl Writable for ItemType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32((self.network_id << 16) | (self.metadata_value as i32));
    }
}

impl Readable<ItemType> for ItemType {
    fn read(reader: &mut Reader) -> Self {
        let value = reader.var_i32();
        Self {
            network_id: value << 16,
            metadata_value: (value & 0x7fff) as u32,
        }
    }
}

#[proto]
#[derive(Debug, Clone)]
pub struct MaterialReducerOutput {
    pub network_id: VarI32,
    pub count: VarI32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PotionContainerChangeRecipe {
    pub input_item_id: VarI32,
    pub reagent_item_id: VarI32,
    pub output_item_id: VarI32,
}

pub type ShulkerBoxRecipe = ShapelessRecipe;
