use std::fmt::Debug;

use bytes::Bytes;
use glam::{IVec3, Vec3};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::encodable_enum;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::item::ItemInstance;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Window {
    Inventory = 0,
    OffHand = 119,
    Armour = 120,
    UI = 124,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum InventoryActionSource {
    Container = 0,
    World = 2,
    Creative = 3,
    TODO = 99999,
}

#[derive(Debug, Clone)]
pub struct InventoryAction {
    pub source_type: InventoryActionSource,
    pub window: Window,
    pub source_flags: u32,
    pub inventory_slot: u32,
    pub old_item: ItemInstance,
    pub new_item: ItemInstance,
}

impl InventoryAction {
    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.source_type.to_u32().unwrap());
        match self.source_type {
            InventoryActionSource::Container | InventoryActionSource::TODO => {
                writer.var_i32(self.window.to_i32().unwrap());
            }
            InventoryActionSource::World => {
                writer.var_u32(self.source_flags);
            }
            _ => {}
        }
        writer.var_u32(self.inventory_slot);
        self.old_item.write(writer);
        self.new_item.write(writer);
    }

    pub fn read(reader: &mut Reader) -> Self {
        let source_type = InventoryActionSource::from_u32(reader.var_u32()).unwrap();
        Self {
            source_type: source_type.clone(),
            window: if source_type == InventoryActionSource::Container || source_type == InventoryActionSource::TODO {
                Window::from_i32(reader.var_i32()).unwrap()
            } else {
                Window::Inventory
            },
            source_flags: if source_type == InventoryActionSource::World {
                reader.var_u32()
            } else {
                0
            },
            inventory_slot: reader.var_u32(),
            old_item: ItemInstance::read(reader),
            new_item: ItemInstance::read(reader),
        }
    }
}

impl Default for InventoryAction {
    fn default() -> Self {
        Self {
            source_type: InventoryActionSource::Container,
            window: Window::Inventory,
            source_flags: 0,
            inventory_slot: 0,
            old_item: ItemInstance::default(),
            new_item: ItemInstance::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LegacySetItemSlot {
    pub container_id: u8,
    pub slots: Bytes,
}

impl LegacySetItemSlot {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(self.container_id);
        writer.byte_slice(&self.slots);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            container_id: reader.u8(),
            slots: reader.byte_slice(),
        }
    }
}

encodable_enum!(
    #[derive(Debug, Clone)]
    pub enum InventoryTransactionData {
        NormalTransaction = 0,
        MismatchTransaction = 1,
        UseItemTransactionData = 2,
        UseItemOnEntityTransaction = 3,
        ReleaseItemTransaction = 4,
    }
);

#[derive(Debug, Clone)]
pub struct MismatchTransaction;

impl MismatchTransaction {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug, Clone)]
pub struct NormalTransaction {}

impl NormalTransaction {
    pub fn read(_: &mut Reader) -> Self {
        Self {}
    }

    pub fn write(&self, _: &mut Writer) {}
}

#[derive(Debug, Clone)]
pub struct ReleaseItemTransaction {
    pub action_type: ReleaseItemAction,
    pub hot_bar_slot: i32,
    pub held_item: ItemInstance,
    pub head_position: Vec3,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum ReleaseItemAction {
    Release,
    Consume,
}

impl ReleaseItemTransaction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: ReleaseItemAction::from_u32(reader.var_u32()).unwrap(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            head_position: reader.vec3(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.action_type.to_u32().unwrap());
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.head_position);
    }
}

#[derive(Debug, Clone)]
pub struct UseItemOnEntityTransaction {
    pub target_entity_runtime_id: u64,
    pub action_type: u32,
    pub hot_bar_slot: i32,
    pub held_item: ItemInstance,
    pub position: Vec3,
    pub clicked_position: Vec3,
}

impl UseItemOnEntityTransaction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            target_entity_runtime_id: reader.var_u64(),
            action_type: reader.var_u32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            position: reader.vec3(),
            clicked_position: reader.vec3(),
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.target_entity_runtime_id);
        writer.var_u32(self.action_type);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.position);
        writer.vec3(self.clicked_position);
    }
}

#[derive(Debug, Clone, Default)]
pub struct UseItemTransactionData {
    pub legacy_request_id: i32,
    pub legacy_set_item_slots: Vec<LegacySetItemSlot>,
    pub actions: Vec<InventoryAction>,
    pub action_type: u32,
    pub block_position: IVec3,
    pub block_face: i32,
    pub hot_bar_slot: i32,
    pub held_item: ItemInstance,
    pub position: Vec3,
    pub clicked_position: Vec3,
    pub block_runtime_id: u32,
}

impl UseItemTransactionData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_u32(),
            block_position: reader.u_block_pos(),
            block_face: reader.var_i32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            position: reader.vec3(),
            clicked_position: reader.vec3(),
            block_runtime_id: reader.var_u32(),
            ..Default::default()
        }
    }

    pub fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.action_type);
        writer.u_block_pos(self.block_position);
        writer.var_i32(self.block_face);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.position);
        writer.vec3(self.clicked_position);
        writer.var_u32(self.block_runtime_id);
    }

    pub fn write_player_action(&self, writer: &mut Writer) {
        writer.var_i32(self.legacy_request_id);
        if self.legacy_request_id < -1 && (self.legacy_request_id & 1) == 0 {
            writer.var_u32(self.legacy_set_item_slots.len() as u32);
            self.legacy_set_item_slots.iter().for_each(|slot| slot.write(writer));
        }
        writer.var_u32(self.actions.len() as u32);
        self.actions.iter().for_each(|action| action.write(writer));
        writer.var_u32(self.action_type);
        writer.block_pos(self.block_position);
        writer.var_i32(self.block_face);
        writer.var_i32(self.hot_bar_slot);
        self.held_item.write(writer);
        writer.vec3(self.position);
        writer.vec3(self.clicked_position);
        writer.var_u32(self.block_runtime_id);
    }

    pub fn read_player_action(reader: &mut Reader) -> Self {
        let legacy_request_id = reader.var_i32();
        Self {
            legacy_request_id,
            legacy_set_item_slots: if legacy_request_id < -1 && (legacy_request_id & 1) == 0 {
                (0..reader.var_u32()).map(|_| LegacySetItemSlot::read(reader)).collect()
            } else {
                Vec::new()
            },
            actions: (0..reader.var_u32()).map(|_| InventoryAction::read(reader)).collect(),
            action_type: reader.var_u32(),
            block_position: reader.block_pos(),
            block_face: reader.var_i32(),
            hot_bar_slot: reader.var_i32(),
            held_item: ItemInstance::read(reader),
            position: reader.vec3(),
            clicked_position: reader.vec3(),
            block_runtime_id: reader.var_u32(),
        }
    }
}
