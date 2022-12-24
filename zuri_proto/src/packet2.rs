use std::collections::BTreeMap;
use bytes::Bytes;
use glam::{IVec2, Vec2, Vec3};
use uuid::Uuid;
use crate::data::*;
use crate::enums::*;
use crate::io::{Reader, Writer};

/// Sent by the server to send a player animation from one player to all viewers of that player. It is used for a couple
/// of actions, such as arm swimming and critical hits.
#[derive(Debug)]
pub struct Animate {
    /// The action type to execute.
    pub action_type: AnimateAction,
    /// The runtime ID of the player that the animation should be played upon. The runtime ID is unique for each world
    /// session, and entities are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// It is unclear what this field does.
    pub boat_rowing_time: f32,
}

impl Packet for Animate {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.action_type).unwrap());
        writer.var_u64(self.entity_runtime_id);
        match self.action_type {
            AnimateAction::RowRight | AnimateAction::RowLeft => {
                writer.f32(self.boat_rowing_time);
            }
            _ => {}
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_i32(reader.var_i32()).unwrap();
        Self {
            action_type,
            entity_runtime_id: reader.var_u64(),
            boat_rowing_time: if action_type == AnimateAction::RowRight || action_type == AnimateAction::RowLeft {
                reader.f32()
            } else {
                0.0
            },
        }
    }
}

/// Sent by the server to make a player respawn client-side. It is sent in response to a PlayerAction packet with the
/// action type Respawn. As of 1.13, the server sends two of these packets with different states, and the client sends
/// one of these back in order to complete the respawn.
#[derive(Debug)]
pub struct Respawn {
    /// The position on which the player should be respawned. The position might be in a different dimension, in which
    /// case the client should first be sent a ChangeDimension packet.
    pub position: Vec3,
    /// The 'state' of the respawn. It is one of the constants that may be found above, and the value the packet
    /// contains depends on whether the server or client sends it.
    pub state: RespawnState,
    /// The entity runtime ID of the player that the respawn packet concerns. This is apparently for the server to
    /// recognise which player sends this packet.
    pub entity_runtime_id: u64,
}

impl Packet for Respawn {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.u8(num::ToPrimitive::to_u8(&self.state).unwrap());
        writer.var_u64(self.entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            state: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            entity_runtime_id: reader.var_u64(),
        }
    }
}

/// Sent by the server to open a container client-side. This container must be physically present in the world, for the
/// packet to have any effect. Unlike Java Edition, Bedrock Edition requires that chests for example must be present and
/// in range to open its inventory.
#[derive(Debug)]
pub struct ContainerOpen {
    /// The window that is being opened. It may be used later to close the container using a ContainerClose packet.
    pub window: Window,
    /// The type of the container that is being opened when opening the container at the position of the packet. It
    /// depends on the block/entity, and could, for example, be a chest or a hopper, but also a horse inventory.
    pub container_type: ContainerType,
    /// The position of the container opened. The position must point to a block entity that actually has a container.
    /// If that is not the case, the window will not be opened and the packet will be ignored, if a valid
    /// container entity unique id has not also been provided.
    pub container_position: BlockPos,
    /// The unique ID of the entity container that was opened. It is only used if the ContainerType is one that points
    /// to an entity, for example a horse.
    pub container_entity_unique_id: i64,
}

impl Packet for ContainerOpen {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.u8(num::ToPrimitive::to_u8(&self.container_type).unwrap());
        writer.u_block_pos(self.container_position);
        writer.var_i64(self.container_entity_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            container_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            container_position: reader.u_block_pos(),
            container_entity_unique_id: reader.var_i64(),
        }
    }
}

/// Sent by the server to close a container the player currently has opened, which was opened using the ContainerOpen
/// packet, or by the client to tell the server it closed a particular container, such as the crafting grid.
#[derive(Debug)]
pub struct ContainerClose {
    /// The window of the container that should be closed. It must be equal to the one sent in the ContainerOpen packet
    /// to close the designated window.
    pub window: Window,
    /// Determines whether or not the container was force-closed by the server. If this value is not set correctly, the
    /// client may ignore the packet and respond with a PacketViolationWarning.
    pub server_side: bool,
}

impl Packet for ContainerClose {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.bool(self.server_side);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            server_side: reader.bool(),
        }
    }
}

/// Sent by the server to the client. It used to be used to link hot bar slots of the player to actual slots in the
/// inventory, but as of 1.2, this was changed and hot bar slots are no longer a free floating part of the inventory.
/// Since 1.2, the packet has been re-purposed, but its new functionality is not clear.
#[derive(Debug)]
pub struct PlayerHotBar {
    /// Before 1.2, this was the hot bar slot that is being linked to the inventory slot.
    pub selected_hotbar_slot: u32,
    /// The window that the hot bar slot is in.
    pub window: Window,
    /// The exact purpose of this field is unknown.
    pub select_hotbar_slot: bool,
}

impl Packet for PlayerHotBar {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.selected_hotbar_slot);
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.bool(self.select_hotbar_slot);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            selected_hotbar_slot: reader.var_u32(),
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            select_hotbar_slot: reader.bool(),
        }
    }
}

/// Sent by the server to update the full content of a particular inventory. It is usually sent for the main inventory
/// of the player, but also works for other inventories that are currently opened by the player.
#[derive(Debug)]
pub struct InventoryContent {
    /// One of the windows that the client currently has opened, or a consistent one such as the main inventory.
    pub window: Window,
    /// The new content of the inventory. The length of this slice must be equal to the full size of the inventory
    /// window that was updated.
    pub content: Vec<ItemInstance>,
}

impl Packet for InventoryContent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(num::ToPrimitive::to_u32(&self.window).unwrap());

        writer.var_u32(self.content.len() as u32);
        self.content.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
            content: (0..reader.var_u32()).map(|_| ItemInstance::read(reader)).collect(),
        }
    }
}

/// Sent by the server to update a single slot in one of the inventory windows that the client currently has opened.
/// Usually this is the main inventory, but it may also be the off hand or, for example, a chest inventory.
#[derive(Debug)]
pub struct InventorySlot {
    /// The window that the packet modifies. It must point to one of the windows that the client currently has opened.
    pub window: Window,
    /// The index of the slot that the packet modifies. The new item will be set to the slot at this index.
    pub slot: u32,
    /// The item to be put in the slot. It will overwrite any item that may currently be present in that slot.
    pub new_item: ItemInstance,
}

impl Packet for InventorySlot {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(num::ToPrimitive::to_u32(&self.window).unwrap());
        writer.var_u32(self.slot);
        self.new_item.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
            slot: reader.var_u32(),
            new_item: ItemInstance::read(reader),
        }
    }
}

/// Sent by the server to update specific data of a single container, meaning a block such as a furnace or a brewing
/// stand. This data is usually used by the client to display certain features client-side.
#[derive(Debug)]
pub struct ContainerSetData {
    /// The window that the packet modifies. It must point to one of the windows that the client currently has opened.
    pub window: Window,
    /// The key of the property. It is one of the constants that can be found above. Multiple properties share the same
    /// key, but the functionality depends on the type of the container that the data is set to.
    pub key: ContainerDataKey,
    /// The value of the property. Its use differs per property.
    pub value: i32,
}

impl Packet for ContainerSetData {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.var_i32(self.key.0);
        writer.var_i32(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            key: ContainerDataKey(reader.var_i32()),
            value: reader.var_i32(),
        }
    }
}

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

/// Sent by the client when it crafts a particular item. Note that this packet may be fully ignored, as the transaction
/// systems provide all the information necessary.
#[derive(Debug)]
pub struct CraftingEvent {
    /// The window that the player crafted in.
    pub window: Window,
    /// The container type of the window the player crafted in.
    pub container_type: ContainerType,
    /// The UUID of the recipe that was crafted. It is the UUID of the recipe that was sent in the CraftingData packet.
    pub recipe_uuid: Uuid,
    /// List of items that the player put into the recipe so that it could create the output items. These items are
    /// consumed in the process.
    pub input: Vec<ItemInstance>,
    /// List of items that were obtained as a result of crafting the recipe.
    pub output: Vec<ItemInstance>,
}

impl Packet for CraftingEvent {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.container_type).unwrap());
        writer.uuid(self.recipe_uuid);

        writer.var_u32(self.input.len() as u32);
        self.input.iter().for_each(|item| item.write(writer));

        writer.var_u32(self.output.len() as u32);
        self.output.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            container_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            recipe_uuid: reader.uuid(),
            input: (0..reader.var_u32()).map(|_| ItemInstance::read(reader)).collect(),
            output: (0..reader.var_u32()).map(|_| ItemInstance::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct GUIDataPickItem {
    pub item_name: String,
    pub item_effects: String,
    pub hot_bar_slot: i32,
}

impl Packet for GUIDataPickItem {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.item_name.as_str());
        writer.string(self.item_effects.as_str());
        writer.i32(self.hot_bar_slot);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            item_name: reader.string(),
            item_effects: reader.string(),
            hot_bar_slot: reader.i32(),
        }
    }
}

#[derive(Debug)]
pub struct AdventureSettings {
    pub flags: u32,
    pub command_permission_level: CommandPermissionLevel,
    pub action_permissions: u32,
    pub permission_level: PermissionLevel,
    pub custom_stored_permissions: u32,
    pub player_unique_id: i64,
}

impl Packet for AdventureSettings {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.flags);
        writer.var_u32(num::ToPrimitive::to_u32(&self.command_permission_level).unwrap());
        writer.var_u32(self.action_permissions);
        writer.var_u32(num::ToPrimitive::to_u32(&self.permission_level).unwrap());
        writer.var_u32(self.custom_stored_permissions);
        writer.i64(self.player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            flags: reader.var_u32(),
            command_permission_level: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
            action_permissions: reader.var_u32(),
            permission_level: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
            custom_stored_permissions: reader.var_u32(),
            player_unique_id: reader.i64(),
        }
    }
}

#[derive(Debug)]
pub struct BlockActorData {
    pub position: BlockPos,
    // pub nbt_data: dyn Any, // TODO: NBT
}

impl Packet for BlockActorData {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        // TODO: NBT (nbt_data)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            // nbt_data: {
            //     // TODO: NBT
            // },
        }
    }
}

#[derive(Debug)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub jumping: bool,
    pub sneaking: bool,
}

impl Packet for PlayerInput {
    fn write(&self, writer: &mut Writer) {
        writer.vec2(self.movement);
        writer.bool(self.jumping);
        writer.bool(self.sneaking);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            movement: reader.vec2(),
            jumping: reader.bool(),
            sneaking: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct LevelChunk {
    pub position: IVec2,
    pub sub_chunk_request_mode: SubChunkRequestMode,
    pub highest_sub_chunk: u16,
    pub sub_chunk_count: u32,
    pub cache_enabled: bool,
    pub blob_hashes: Vec<u64>,
    pub raw_payload: Bytes,
}

impl Packet for LevelChunk {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.position.x);
        writer.var_i32(self.position.y);
        match self.sub_chunk_request_mode {
            SubChunkRequestMode::Legacy => {
                writer.var_u32(self.sub_chunk_count);
            }
            SubChunkRequestMode::Limitless => {
                writer.var_u32(u32::MAX);
            }
            SubChunkRequestMode::Limited => {
                writer.var_u32(u32::MAX - 1);
                writer.u16(self.highest_sub_chunk);
            }
        }
        writer.bool(self.cache_enabled);
        if self.cache_enabled {
            writer.var_u32(self.blob_hashes.len() as u32);
            self.blob_hashes.iter().for_each(|hash| writer.u64(*hash));
        }
        writer.byte_slice(&self.raw_payload);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            position: IVec2::new(reader.var_i32(), reader.var_i32()),
            sub_chunk_request_mode: SubChunkRequestMode::Legacy,
            highest_sub_chunk: 0,
            sub_chunk_count: 0,
            cache_enabled: false,
            blob_hashes: Vec::new(),
            raw_payload: Bytes::default(),
        };
        let sub_chunk_count = reader.var_u32();
        if sub_chunk_count == u32::MAX {
            packet.sub_chunk_request_mode = SubChunkRequestMode::Limitless;
        } else if sub_chunk_count == u32::MAX - 1 {
            packet.sub_chunk_request_mode = SubChunkRequestMode::Limited;
            packet.highest_sub_chunk = reader.u16();
        } else {
            packet.sub_chunk_count = sub_chunk_count;
        }
        packet.cache_enabled = reader.bool();
        if packet.cache_enabled {
            let blob_hashes_len = reader.var_u32() as usize;
            packet.blob_hashes = (0..blob_hashes_len).map(|_| reader.u64()).collect();
        }
        packet.raw_payload = reader.byte_slice();

        packet
    }
}

#[derive(Debug)]
pub struct SetCommandsEnabled {
    pub enabled: bool,
}

impl Packet for SetCommandsEnabled {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.enabled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            enabled: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct SetDifficulty {
    pub difficulty: Difficulty,
}

impl Packet for SetDifficulty {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(num::ToPrimitive::to_u32(&self.difficulty).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            difficulty: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ChangeDimension {
    pub dimension: Dimension,
    pub position: Vec3,
    pub respawn: bool,
}

impl Packet for ChangeDimension {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.vec3(self.position);
        writer.bool(self.respawn);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.vec3(),
            respawn: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct SetPlayerGameType {
    pub game_type: GameType,
}

impl Packet for SetPlayerGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.game_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct PlayerList {
    pub action_type: PlayerListAction,
    pub entries: Vec<PlayerListEntry>,
}

impl Packet for PlayerList {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());
        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| PlayerListEntry::read(reader, action_type)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct SimpleEvent {
    pub event_type: i16,
}

impl Packet for SimpleEvent {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.event_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_type: reader.i16(),
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub entity_runtime_id: u64,
    pub use_player_id: u8,
    pub event_data: EventData2,
}

impl Packet for Event {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        // todo: THIS DOESNT WORK BECAUSE OF THE FUCKING USE_PLAYER_ID
        //writer.var_i32(num::ToPrimitive::to_i32(&self.event_data.event_type()).unwrap());
        writer.u8(self.use_player_id);
        //self.event_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        let entity_runtime_id = reader.var_u64();
        // todo: FUCJKING USE_POLAYER_ID @#G@O*GF)*@GV#
        //let event_type = num::FromPrimitive::from_i32(reader.var_i32()).unwrap();
        Self {
            entity_runtime_id,
            use_player_id: reader.u8(),
            event_data: EventData2::read(reader),
        }
    }
}

#[derive(Debug)]
pub struct SpawnExperienceOrb {
    pub position: Vec3,
    pub experience_amount: i32,
}

impl Packet for SpawnExperienceOrb {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.var_i32(self.experience_amount);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            experience_amount: reader.var_i32(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ClientBoundMapItemData {
    pub map_id: i64,
    pub update_flags: u32,
    pub dimension: u8,
    pub locked_map: bool,
    pub origin: BlockPos,
    pub scale: u8,
    pub maps_included_in: Vec<i64>,
    pub tracked_objects: Vec<MapTrackedObject>,
    pub decorations: Vec<MapDecoration>,
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub pixels: Vec<RGBA>,
}

impl Packet for ClientBoundMapItemData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.map_id);
        writer.var_u32(self.update_flags);
        writer.u8(self.dimension);
        writer.bool(self.locked_map);
        writer.block_pos(self.origin);

        if self.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            writer.var_u32(self.maps_included_in.len() as u32);
            self.maps_included_in.iter().for_each(|map_id| { writer.var_i64(*map_id); });
        }
        if self.update_flags & (MapUpdateFlag::Initialisation.flag() | MapUpdateFlag::Decoration.flag() | MapUpdateFlag::Texture.flag()) != 0 {
            writer.u8(self.scale);
        }
        if self.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            writer.var_u32(self.tracked_objects.len() as u32);
            self.tracked_objects.iter().for_each(|tracked_object| tracked_object.write(writer));
            writer.var_u32(self.decorations.len() as u32);
            self.decorations.iter().for_each(|decoration| decoration.write(writer));
        }
        if self.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            writer.i32(self.width);
            writer.i32(self.height);
            writer.i32(self.x_offset);
            writer.i32(self.y_offset);
            writer.var_u32(self.pixels.len() as u32);
            self.pixels.iter().for_each(|pixels| pixels.write_var(writer));
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            map_id: reader.var_i64(),
            update_flags: reader.var_u32(),
            dimension: reader.u8(),
            locked_map: reader.bool(),
            origin: reader.block_pos(),
            ..Default::default()
        };
        if packet.update_flags & MapUpdateFlag::Initialisation.flag() != 0 {
            packet.maps_included_in = (0..reader.var_u32()).map(|_| reader.var_i64()).collect();
        }
        if packet.update_flags & (MapUpdateFlag::Initialisation.flag() | MapUpdateFlag::Decoration.flag() | MapUpdateFlag::Texture.flag()) != 0 {
            packet.scale = reader.u8();
        }
        if packet.update_flags & MapUpdateFlag::Decoration.flag() != 0 {
            packet.tracked_objects = (0..reader.var_u32()).map(|_| MapTrackedObject::read(reader)).collect();
            packet.decorations = (0..reader.var_u32()).map(|_| MapDecoration::read(reader)).collect();
        }
        if packet.update_flags & MapUpdateFlag::Texture.flag() != 0 {
            packet.width = reader.i32();
            packet.height = reader.i32();
            packet.x_offset = reader.i32();
            packet.y_offset = reader.i32();
            packet.pixels = (0..reader.var_u32()).map(|_| RGBA::read_var(reader)).collect();
        }

        packet
    }
}

#[derive(Debug)]
pub struct MapInfoRequest {
    pub map_id: i64,
    pub client_pixels: Vec<PixelRequest>,
}

impl Packet for MapInfoRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.map_id);
        writer.var_u32(self.client_pixels.len() as u32);
        self.client_pixels.iter().for_each(|pixel| pixel.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            map_id: reader.var_i64(),
            client_pixels: (0..reader.var_u32()).map(|_| PixelRequest::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct RequestChunkRadius {
    pub chunk_radius: i32,
}

impl Packet for RequestChunkRadius {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.chunk_radius);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            chunk_radius: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct ChunkRadiusUpdated {
    pub chunk_radius: i32,
}

impl Packet for ChunkRadiusUpdated {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.chunk_radius);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            chunk_radius: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct ItemFrameDropItem {
    pub position: BlockPos,
}

impl Packet for ItemFrameDropItem {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
        }
    }
}

#[derive(Debug)]
pub struct GameRulesChanged {
    pub game_rules: Vec<GameRule>,
}

impl Packet for GameRulesChanged {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.game_rules.len() as u32);
        self.game_rules.iter().for_each(|game_rule| game_rule.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { game_rules: (0..reader.var_u32()).map(|_| GameRule::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub camera_entity_unique_id: i64,
    pub target_player_unique_id: i64,
}

impl Packet for Camera {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.camera_entity_unique_id);
        writer.var_i64(self.target_player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            camera_entity_unique_id: reader.var_i64(),
            target_player_unique_id: reader.var_i64(),
        }
    }
}

#[derive(Debug)]
pub struct BossEvent {
    pub boss_entity_unique_id: i64,
    pub event_type: BossEventType,
    pub player_unique_id: i64,
    pub boss_bar_title: String,
    pub health_percentage: f32,
    pub screen_darkening: i16,
    pub colour: u32,
    pub overlay: u32,
}

impl Packet for BossEvent {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.boss_entity_unique_id);
        writer.u32(num::ToPrimitive::to_u32(&self.event_type).unwrap());
        match self.event_type {
            BossEventType::Show => {
                writer.string(self.boss_bar_title.as_str());
                writer.f32(self.health_percentage);
                writer.i16(self.screen_darkening);
                writer.u32(self.colour);
                writer.u32(self.overlay);
            }
            BossEventType::RegisterPlayer | BossEventType::UnregisterPlayer | BossEventType::Request => {
                writer.i64(self.player_unique_id);
            }
            BossEventType::Hide => {}
            BossEventType::HealthPercentage => {
                writer.f32(self.health_percentage);
            }
            BossEventType::Title => {
                writer.string(self.boss_bar_title.as_str());
            }
            BossEventType::AppearanceProperties => {
                writer.i16(self.screen_darkening);
                writer.u32(self.colour);
                writer.u32(self.overlay);
            }
            BossEventType::Texture => {
                writer.u32(self.colour);
                writer.u32(self.overlay);
            }
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let boss_entity_unique_id = reader.i64();
        let event_type = num::FromPrimitive::from_u32(reader.u32()).unwrap();
        Self {
            boss_entity_unique_id,
            event_type,
            player_unique_id: if event_type == BossEventType::RegisterPlayer || event_type == BossEventType::UnregisterPlayer || event_type == BossEventType::Request {
                reader.i64()
            } else {
                0
            },
            boss_bar_title: if event_type == BossEventType::Show || event_type == BossEventType::Title { reader.string() } else { "".to_string() },
            health_percentage: if event_type == BossEventType::Show || event_type == BossEventType::HealthPercentage { reader.f32() } else { 0.0 },
            screen_darkening: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties { reader.i16() } else { 0 },
            colour: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties || event_type == BossEventType::Texture {
                reader.u32()
            } else {
                0
            },
            overlay: if event_type == BossEventType::Show || event_type == BossEventType::AppearanceProperties || event_type == BossEventType::Texture {
                reader.u32()
            } else {
                0
            },
        }
    }
}

#[derive(Debug)]
pub struct ShowCredits {
    pub player_runtime_id: u64,
    pub status_type: i32,
}

impl Packet for ShowCredits {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.player_runtime_id);
        writer.var_i32(self.status_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_runtime_id: reader.var_u64(),
            status_type: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct AvailableCommands {
    pub commands: Vec<Command>,
    pub constraints: Vec<CommandEnumConstraint>,
}

impl AvailableCommands {
    fn enum_values(&self) -> (Vec<String>, BTreeMap<String, usize>) {
        let mut values = Vec::new();
        let mut indices = BTreeMap::new();
        for command in &self.commands {
            for alias in &command.aliases {
                if !indices.contains_key(alias.as_str()) {
                    indices.insert(alias.clone(), values.len());
                    values.push(alias.clone());
                }
            }
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    for option in &parameter.command_enum.options {
                        if !indices.contains_key(option) {
                            indices.insert(option.clone(), values.len());
                            values.push(option.clone());
                        }
                    }
                }
            }
        }

        (values, indices)
    }

    fn suffixes(&self) -> (Vec<String>, BTreeMap<String, usize>) {
        let mut values = Vec::new();
        let mut indices = BTreeMap::new();
        for command in &self.commands {
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if !parameter.suffix.is_empty() {
                        if !indices.contains_key(&parameter.suffix) {
                            indices.insert(parameter.suffix.clone(), values.len());
                            values.push(parameter.suffix.clone());
                        }
                    }
                }
            }
        }

        (values, indices)
    }

    fn enums(&self) -> (Vec<CommandEnum>, BTreeMap<String, usize>) {
        todo!()
        // let mut values = Vec::new();
        // let mut indices = BTreeMap::new();
        // for command in self.commands {
        //     if !command.aliases.is_empty() {
        //         let alias_enum = CommandEnum {
        //             enum_type: format!("{}Aliases", command.name),
        //             options: command.aliases,
        //             ..Default::default()
        //         };
        //         indices.insert(alias_enum.enum_type, values.len());
        //         values.push(alias_enum);
        //     }
        //     for overload in command.overloads {
        //         for parameter in overload.parameters {
        //             if !parameter.command_enum.options.is_empty() && !parameter.command_enum.dynamic {
        //                 if !indices.contains_key(&parameter.command_enum.enum_type) {
        //                     indices.insert(parameter.command_enum.enum_type.clone(), values.len());
        //                     values.push(parameter.command_enum);
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // (values, indices)
    }

    fn dynamic_enums(&self) -> (Vec<CommandEnum>, BTreeMap<String, usize>) {
        let mut values = Vec::new();
        let mut indices = BTreeMap::new();
        for command in &self.commands {
            for overload in &command.overloads {
                for parameter in &overload.parameters {
                    if parameter.command_enum.dynamic {
                        if !indices.contains_key(&parameter.command_enum.enum_type) {
                            indices.insert(parameter.command_enum.enum_type.clone(), values.len());
                            values.push(parameter.command_enum.clone());
                        }
                    }
                }
            }
        }

        (values, indices)
    }
}

impl Packet for AvailableCommands {
    fn write(&self, writer: &mut Writer) {
        todo!()
        // (values, valueIndices) = self.enum_values();
        // (suffixes, suffixIndices) = self.suffixes();
        // (enums, enumIndices) = self.enums();
        // (dynamicEnums, dynamicEnumIndices) = self.dynamic_enums();
        //
        // writer.var_u32(values.len() as u32);
        // values.iter().for_each(|value| writer.string(value));
        // writer.var_u32(suffixes.len() as u32);
        // suffixes.iter().for_each(|suffix| writer.string(suffix));
        //
        // writer.var_u32(enums.len() as u32);
        // enums.iter().for_each(|command_enum| command_enum.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        todo!()
        // Self {
        //     LEN: reader.read_TODO(),
        //     commands: reader.read_Command(),
        //     LEN: reader.read_TODO(),
        //     constraints: reader.read_CommandEnumConstraint(),
        // }
    }
}

#[derive(Debug)]
pub struct CommandRequest {
    pub command_line: String,
    pub command_origin: CommandOrigin,
    pub internal: bool,
}

impl Packet for CommandRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.command_line.as_str());
        self.command_origin.write(writer);
        writer.bool(self.internal);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            command_line: reader.string(),
            command_origin: CommandOrigin::read(reader),
            internal: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct CommandBlockUpdate {
    pub block: bool,
    pub position: BlockPos,
    pub mode: u32,
    pub needs_redstone: bool,
    pub conditional: bool,
    pub minecart_entity_runtime_id: u64,
    pub command: String,
    pub last_output: String,
    pub name: String,
    pub should_track_output: bool,
    pub tick_delay: i32,
    pub execute_on_first_tick: bool,
}

impl Packet for CommandBlockUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.block);
        if self.block {
            writer.u_block_pos(self.position);
            writer.var_u32(self.mode);
            writer.bool(self.needs_redstone);
            writer.bool(self.conditional);
        } else {
            writer.u64(self.minecart_entity_runtime_id);
        }
        writer.string(self.command.as_str());
        writer.string(self.last_output.as_str());
        writer.string(self.name.as_str());
        writer.bool(self.should_track_output);
        writer.i32(self.tick_delay);
        writer.bool(self.execute_on_first_tick);
    }

    fn read(reader: &mut Reader) -> Self {
        let block = reader.bool();
        Self {
            block,
            position: if block { reader.u_block_pos() } else { BlockPos::default() },
            mode: if block { reader.var_u32() } else { 0 },
            needs_redstone: if block { reader.bool() } else { false },
            conditional: if block { reader.bool() } else { false },
            minecart_entity_runtime_id: if !block { reader.u64() } else { 0 },
            command: reader.string(),
            last_output: reader.string(),
            name: reader.string(),
            should_track_output: reader.bool(),
            tick_delay: reader.i32(),
            execute_on_first_tick: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct CommandOutput {
    pub command_origin: CommandOrigin,
    pub output_type: CommandOutputType,
    pub success_count: u32,
    pub output_messages: Vec<CommandOutputMessage>,
    pub data_set: String,
}

impl Packet for CommandOutput {
    fn write(&self, writer: &mut Writer) {
        self.command_origin.write(writer);
        writer.u8(num::ToPrimitive::to_u8(&self.output_type).unwrap());
        writer.var_u32(self.success_count);

        writer.var_u32(self.output_messages.len() as u32);
        self.output_messages.iter().for_each(|message| message.write(writer));

        if self.output_type == CommandOutputType::DataSet {
            writer.string(self.data_set.as_str());
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let command_origin = CommandOrigin::read(reader);
        let output_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            command_origin,
            output_type,
            success_count: reader.var_u32(),
            output_messages: (0..reader.var_u32()).map(|_| CommandOutputMessage::read(reader)).collect(),
            data_set: if output_type == CommandOutputType::DataSet { reader.string() } else { "".to_string() },
        }
    }
}

#[derive(Debug)]
pub struct UpdateTrade {
    pub window: Window,
    pub window_type: u8,
    pub size: i32,
    pub trade_tier: i32,
    pub villager_unique_id: i64,
    pub entity_unique_id: i64,
    pub display_name: String,
    pub new_trade_ui: bool,
    pub demand_based_prices: bool,
    pub serialised_offers: Bytes,
}

impl Packet for UpdateTrade {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.u8(self.window_type);
        writer.var_i32(self.size);
        writer.var_i32(self.trade_tier);
        writer.var_i64(self.villager_unique_id);
        writer.var_i64(self.entity_unique_id);
        writer.string(self.display_name.as_str());
        writer.bool(self.new_trade_ui);
        writer.bool(self.demand_based_prices);
        writer.byte_slice(&self.serialised_offers);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            window_type: reader.u8(),
            size: reader.var_i32(),
            trade_tier: reader.var_i32(),
            villager_unique_id: reader.var_i64(),
            entity_unique_id: reader.var_i64(),
            display_name: reader.string(),
            new_trade_ui: reader.bool(),
            demand_based_prices: reader.bool(),
            serialised_offers: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateEquip {
    pub window: Window,
    pub window_type: u8,
    pub size: i32,
    pub entity_unique_id: i64,
    pub serialised_inventory_data: Bytes,
}

impl Packet for UpdateEquip {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.window).unwrap());
        writer.u8(self.window_type);
        writer.var_i32(self.size);
        writer.var_i64(self.entity_unique_id);
        writer.bytes(&self.serialised_inventory_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            window_type: reader.u8(),
            size: reader.var_i32(),
            entity_unique_id: reader.var_i64(),
            serialised_inventory_data: reader.bytes(),
        }
    }
}

#[derive(Debug)]
pub struct ResourcePackDataInfo {
    pub uuid: String,
    pub data_chunk_size: u32,
    pub chunk_count: u32,
    pub size: u64,
    pub hash: Bytes,
    pub premium: bool,
    pub pack_type: ResourcePackType,
}

impl Packet for ResourcePackDataInfo {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.data_chunk_size);
        writer.u32(self.chunk_count);
        writer.u64(self.size);
        writer.byte_slice(&self.hash);
        writer.bool(self.premium);
        writer.u8(num::ToPrimitive::to_u8(&self.pack_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            data_chunk_size: reader.u32(),
            chunk_count: reader.u32(),
            size: reader.u64(),
            hash: reader.byte_slice(),
            premium: reader.bool(),
            pack_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ResourcePackChunkData {
    pub uuid: String,
    pub chunk_index: u32,
    pub data_offset: u64,
    pub data: Bytes,
}

impl Packet for ResourcePackChunkData {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.chunk_index);
        writer.u64(self.data_offset);
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            chunk_index: reader.u32(),
            data_offset: reader.u64(),
            data: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct ResourcePackChunkRequest {
    pub uuid: String,
    pub chunk_index: u32,
}

impl Packet for ResourcePackChunkRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.u32(self.chunk_index);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            chunk_index: reader.u32(),
        }
    }
}

#[derive(Debug)]
pub struct Transfer {
    pub address: String,
    pub port: u16,
}

impl Packet for Transfer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.address.as_str());
        writer.u16(self.port);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            address: reader.string(),
            port: reader.u16(),
        }
    }
}

