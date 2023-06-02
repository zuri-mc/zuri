use bytes::BytesMut;
use num_derive::{FromPrimitive, ToPrimitive};

use zuri_nbt::{encoding::LittleEndian, NBTTag};
use zuri_net_derive::proto;

use crate::proto::io::{Readable, Reader, Writable, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum UseItemAction {
    ClickBlock,
    ClickAir,
    BreakBlock,
}

#[proto(i32)]
#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum UseItemMethod {
    EquipArmour,
    Eat,
    Attack,
    Consume,
    Throw,
    Shoot,
    Place,
    FillBottle,
    FillBucket,
    PourBucket,
    UseTool,
    Interact,
    Retrieved,
    Dyed,
    Traded,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum UseItemOnEntityAction {
    Interact,
    Attack,
}

#[derive(Debug, Clone, Default)]
pub struct ItemInstance {
    pub stack_network_id: i32,
    pub stack: ItemStack,
}

impl Writable for ItemInstance {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.stack.network_id);
        if self.stack.network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return;
        }

        writer.u16(self.stack.count);
        writer.var_u32(self.stack.metadata_value);

        let has_net_id = self.stack_network_id != 0;
        writer.bool(has_net_id);
        if has_net_id {
            writer.var_i32(self.stack_network_id);
        }

        writer.var_i32(self.stack.block_runtime_id);

        let mut extra_data = Writer::default();
        if let NBTTag::Compound(m) = &self.stack.nbt_data {
            if !m.is_empty() {
                extra_data.i16(-1);
                extra_data.u8(1);
                extra_data.nbt(&self.stack.nbt_data, LittleEndian);
            } else {
                extra_data.i16(0);
            }
        } else {
            panic!("nbt data is not a compound tag");
        }

        extra_data.u32(self.stack.can_be_placed_on.len() as u32);
        self.stack
            .can_be_placed_on
            .iter()
            .for_each(|b| extra_data.string_utf(b.as_str()));

        extra_data.u32(self.stack.can_break.len() as u32);
        self.stack
            .can_break
            .iter()
            .for_each(|b| extra_data.string_utf(b.as_str()));

        if self.stack.network_id == writer.shield_id() {
            extra_data.i64(0);
        }

        writer.byte_slice(Into::<BytesMut>::into(extra_data).as_ref());
    }
}

impl Readable<ItemInstance> for ItemInstance {
    fn read(reader: &mut Reader) -> Self {
        let mut instance = Self {
            stack: ItemStack {
                network_id: reader.var_i32(),
                ..Default::default()
            },
            ..Default::default()
        };
        if instance.stack.network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return Self::default();
        }

        instance.stack.count = reader.u16();
        instance.stack.metadata_value = reader.var_u32();
        if reader.bool() {
            instance.stack_network_id = reader.var_i32();
        }
        instance.stack.block_runtime_id = reader.var_i32();

        let mut extra_data = Reader::from_buf(reader.byte_slice(), reader.shield_id());

        let length = extra_data.i16();
        if length == -1 {
            let version = extra_data.u8();
            match version {
                1 => instance.stack.nbt_data = extra_data.nbt(LittleEndian),
                _ => panic!("unknown item user data version {}", version),
            }
        } else if length > 0 {
            instance.stack.nbt_data = extra_data.nbt(LittleEndian);
        }

        instance.stack.can_be_placed_on = (0..extra_data.u32())
            .map(|_| extra_data.string_utf())
            .collect();
        instance.stack.can_break = (0..extra_data.u32())
            .map(|_| extra_data.string_utf())
            .collect();

        if instance.stack.network_id == reader.shield_id() {
            extra_data.i64();
        }

        instance
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemStack {
    pub network_id: i32,
    pub metadata_value: u32,
    pub block_runtime_id: i32,
    pub count: u16,
    pub nbt_data: NBTTag,
    pub can_be_placed_on: Vec<String>,
    pub can_break: Vec<String>,
    pub has_network_id: bool,
}

impl Writable for ItemStack {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.network_id);
        if self.network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return;
        }

        writer.u16(self.count);
        writer.var_u32(self.metadata_value);
        writer.var_i32(self.block_runtime_id);

        let mut extra_data = Writer::default();
        if let NBTTag::Compound(m) = &self.nbt_data {
            if !m.is_empty() {
                extra_data.i16(-1);
                extra_data.u8(1);
                extra_data.nbt(&self.nbt_data, LittleEndian);
            } else {
                extra_data.i16(0);
            }
        } else {
            panic!("nbt data is not a compound tag");
        }

        extra_data.u32(self.can_be_placed_on.len() as u32);
        self.can_be_placed_on
            .iter()
            .for_each(|b| extra_data.string_utf(b.as_str()));

        extra_data.u32(self.can_break.len() as u32);
        self.can_break
            .iter()
            .for_each(|b| extra_data.string_utf(b.as_str()));

        if self.network_id == writer.shield_id() {
            extra_data.i64(0);
        }

        writer.byte_slice(Into::<BytesMut>::into(extra_data).as_ref());
    }
}

impl Readable<ItemStack> for ItemStack {
    fn read(reader: &mut Reader) -> Self {
        let mut stack = Self {
            network_id: reader.var_i32(),
            ..Default::default()
        };
        if stack.network_id == 0 {
            // The item was air, so there's no more data to follow. Return immediately.
            return Self::default();
        }

        stack.count = reader.u16();
        stack.metadata_value = reader.var_u32();
        stack.block_runtime_id = reader.var_i32();

        let mut extra_data = Reader::from_buf(reader.byte_slice(), reader.shield_id());

        let length = extra_data.i16();
        if length == -1 {
            let version = extra_data.u8();
            match version {
                1 => stack.nbt_data = extra_data.nbt(LittleEndian),
                _ => panic!("unknown item user data version {}", version),
            }
        } else if length > 0 {
            stack.nbt_data = extra_data.nbt(LittleEndian);
        }

        stack.can_be_placed_on = (0..extra_data.u32())
            .map(|_| extra_data.string_utf())
            .collect();
        stack.can_break = (0..extra_data.u32())
            .map(|_| extra_data.string_utf())
            .collect();

        if stack.network_id == reader.shield_id() {
            extra_data.i64();
        }

        stack
    }
}
