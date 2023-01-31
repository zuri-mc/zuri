use num_derive::{FromPrimitive, ToPrimitive};
use uuid::Uuid;

use crate::proto::io::{Reader, Writer};
use crate::proto::types::item::ItemStack;
use crate::proto::types::item_descriptor::ItemDescriptorCount;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
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

#[derive(Debug, Clone)]
pub struct MultiRecipe {
    pub uuid: Uuid,
    pub recipe_network_id: u32,
}

impl MultiRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.uuid(self.uuid);
        writer.var_u32(self.recipe_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.uuid(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FurnaceDataRecipe {
    pub furnace_recipe: FurnaceRecipe,
}

impl FurnaceDataRecipe {
    pub fn write(&self, writer: &mut Writer) {
        self.furnace_recipe.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            furnace_recipe: FurnaceRecipe::read(reader),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FurnaceRecipe {
    pub network_id: i32,
    pub output: ItemStack,
    pub block: String,
}

impl FurnaceRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.network_id);
        self.output.write(writer);
        writer.string(self.block.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.var_i32(),
            output: ItemStack::read(reader),
            block: reader.string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PotionRecipe {
    pub input_potion_id: i32,
    pub input_potion_metadata: i32,
    pub reagent_item_id: i32,
    pub reagent_item_metadata: i32,
    pub output_potion_id: i32,
    pub output_potion_metadata: i32,
}

impl PotionRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.input_potion_id);
        writer.var_i32(self.input_potion_metadata);
        writer.var_i32(self.reagent_item_id);
        writer.var_i32(self.reagent_item_metadata);
        writer.var_i32(self.output_potion_id);
        writer.var_i32(self.output_potion_metadata);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            input_potion_id: reader.var_i32(),
            input_potion_metadata: reader.var_i32(),
            reagent_item_id: reader.var_i32(),
            reagent_item_metadata: reader.var_i32(),
            output_potion_id: reader.var_i32(),
            output_potion_metadata: reader.var_i32(),
        }
    }
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

impl ShapedRecipe {
    pub fn write(&self, writer: &mut Writer) {
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

    pub fn read(reader: &mut Reader) -> Self {
        let recipe_id = reader.string();
        let width = reader.i32();
        let height = reader.i32();
        Self {
            recipe_id,
            width,
            height,
            input: (0..width * height).map(|_| ItemDescriptorCount::read(reader)).collect(),
            output: (0..reader.var_u32()).map(|_| ItemStack::read(reader)).collect(),
            uuid: reader.uuid(),
            block: reader.string(),
            priority: reader.var_i32(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

pub type ShapelessChemistryRecipe = ShapelessRecipe;

#[derive(Debug, Clone)]
pub struct ShapelessRecipe {
    pub recipe_id: String,
    pub input: Vec<ItemDescriptorCount>,
    pub output: Vec<ItemStack>,
    pub uuid: Uuid,
    pub block: String,
    pub priority: i32,
    pub recipe_network_id: u32,
}

impl ShapelessRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.recipe_id.as_str());
        writer.var_u32(self.input.len() as u32);
        self.input.iter().for_each(|input| input.write(writer));
        writer.var_u32(self.output.len() as u32);
        self.output.iter().for_each(|stack| stack.write(writer));
        writer.uuid(self.uuid);
        writer.string(self.block.as_str());
        writer.var_i32(self.priority);
        writer.var_u32(self.recipe_network_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            recipe_id: reader.string(),
            input: (0..reader.var_u32()).map(|_| ItemDescriptorCount::read(reader)).collect(),
            output: (0..reader.var_u32()).map(|_| ItemStack::read(reader)).collect(),
            uuid: reader.uuid(),
            block: reader.string(),
            priority: reader.var_i32(),
            recipe_network_id: reader.var_u32(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialReducer {
    pub network_id: i32,
    pub metadata_value: u32,
    pub outputs: Vec<MaterialReducerOutput>,
}

impl MaterialReducer {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32((self.network_id << 16) | (self.metadata_value as i32));
        writer.var_u32(self.outputs.len() as u32);
        self.outputs.iter().for_each(|output| output.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.var_i32() >> 16,
            metadata_value: (reader.var_i32() & 0xFFFF) as u32,
            outputs: (0..reader.var_u32()).map(|_| MaterialReducerOutput::read(reader)).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialReducerOutput {
    pub network_id: i32,
    pub count: i32,
}

impl MaterialReducerOutput {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.network_id);
        writer.var_i32(self.count);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            network_id: reader.var_i32(),
            count: reader.var_i32(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PotionContainerChangeRecipe {
    pub input_item_id: i32,
    pub reagent_item_id: i32,
    pub output_item_id: i32,
}

impl PotionContainerChangeRecipe {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.input_item_id);
        writer.var_i32(self.reagent_item_id);
        writer.var_i32(self.output_item_id);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            input_item_id: reader.var_i32(),
            reagent_item_id: reader.var_i32(),
            output_item_id: reader.var_i32(),
        }
    }
}

pub type ShulkerBoxRecipe = ShapelessRecipe;
