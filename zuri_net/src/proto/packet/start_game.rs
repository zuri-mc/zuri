use glam::Vec3;
use uuid::Uuid;

use crate::proto::ints::{VarI32, VarI64, VarU32, VarU64};
use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;

use crate::proto::io::{UBlockPos, NBT};
use crate::proto::types::education::EducationSharedResourceURI;
use crate::proto::types::game_rule::GameRule;
use crate::proto::types::item_stack::ItemEntry;
use crate::proto::types::player::PlayerMovementSettings;
use crate::proto::types::world::{
    BlockEntry, Difficulty, Dimension, ExperimentData, GameType, Generator, PermissionLevel,
};

/// Sent by the server to send information about the world the player will be spawned in. It
/// contains information about the position the player spawns in, and information about the world in
/// general such as its game rules.
#[proto]
#[derive(Debug, Clone)]
pub struct StartGame {
    /// The unique ID of the player. The unique ID is a value that remains consistent across
    /// different sessions of the same world, but most servers simply fill the runtime ID of the
    /// entity out for this field.
    pub entity_unique_id: VarI64,
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: VarU64,
    /// The game mode of the player. If set to Default, then the client will fall back to the game
    /// mode set in the world_game_mode field, which is the default game mode of the world.
    pub player_game_mode: GameType,
    /// The spawn position of the player in the world. In servers this is often the same as the
    /// world's spawn position found below. Otherwise, this is usually the position the player was
    /// last at when they left the world.
    pub player_position: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of 0. Pitch is
    /// measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The seed used to generate the world.
    pub world_seed: i64,
    /// Specifies if the biome that the player spawns in is user defined (through behaviour packs)
    /// or builtin.
    pub spawn_biome_type: SpawnBiomeType,
    /// A readable name of the biome that the player spawned in, such as 'plains'. This might be a
    /// custom biome name if any custom biomes are present through behaviour packs.
    pub user_defined_biome_name: String,
    /// The dimension that the player spawns in. Most mini-game servers use the Overworld dimension.
    #[enum_header(VarI32)]
    pub dimension: Dimension,
    /// The generator used for the world. Most vanilla worlds use the Overworld generator.
    pub generator: Generator,
    /// The game mode that a player gets when it first spawns in the world. It is shown in the
    /// settings and is used if the player game mode is set to Default.
    pub world_game_mode: GameType,
    /// The difficulty of the world. It is not exactly clear why this is sent to the client, as the
    /// client does not need to know the difficulty of the world.
    #[enum_header(VarI32)]
    pub difficulty: Difficulty,
    /// The block on which the world spawn of the world. This coordinate has no effect on the place
    /// that the client spawns, but it does have an effect on the direction that a compass points.
    pub world_spawn: UBlockPos,
    /// Specifies if achievements are disabled in the world. The client crashes if this value is set
    /// to true while the player's or the world's game mode is creative. It's recommended to simply
    /// always set this to false as a server.
    pub achievements_disabled: bool,
    /// Dictates if the world is in editor mode, a special mode recently introduced adding "powerful
    /// tools for editing worlds, intended for experienced creators."
    pub editor_world_type: EditorWorldType,
    /// Dictates if the world was created as a project in the editor mode. The functionality of this
    /// field is currently unknown.
    pub created_in_editor: bool,
    /// Dictates if the world was exported from editor mode. The functionality of this field is
    /// currently unknown.
    pub exported_from_editor: bool,
    /// The time at which the day cycle was locked if the day cycle is disabled using the respective
    /// game rule. The client will maintain this time as long as the day cycle is disabled.
    pub day_cycle_lock_time: VarI32,
    /// Minecraft: Education Edition field that specifies what 'region' the world is in.
    pub education_edition_offer: EducationEditionRegion,
    /// Specifies if the world has Education Edition features enabled.
    pub education_features_enabled: bool,
    /// UUID used to identify the Education Edition server instance. It is usually unique.
    pub education_product_id: String,
    /// The intensity of the rain falling. When set to 0, no rain falls at all.
    pub rain_level: f32,
    /// The intensity of the thunder. This may actually be set independently from the rain level,
    /// meaning dark clouds can be produced without rain.
    pub lightning_level: f32,
    /// Specifies if the world has platform locked content.
    pub confirmed_platform_locked_content: bool,
    /// Specifies if the world is a multi-player game. This should always be set to true for
    /// servers.
    pub multi_player_game: bool,
    /// Specifies if LAN broadcast was intended to be enabled for the world.
    pub lan_broadcast_enabled: bool,
    /// The mode used to broadcast the joined game across XBOX Live.
    pub xbl_broadcast_mode: GamePublishSetting,
    /// The mode used to broadcast the joined game across the platform.
    pub platform_broadcast_mode: GamePublishSetting,
    /// Specifies if commands are enabled for the player. It is recommended to always set this to
    /// true on a server, as setting it to false means the player cannot, under any circumstance,
    /// use a command.
    pub commands_enabled: bool,
    /// Specifies if the texture pack the world might hold is required, meaning the client was
    /// forced to download it before joining. This is usually set to true for servers.
    pub texture_pack_required: bool,
    /// The game rules currently active with their respective values. The value of these game rules
    /// may be either 'bool', 'i32' or 'j32'. Some game rules are server side only and don't need to
    /// be sent to the client.
    #[len_type(VarU32)]
    pub game_rules: Vec<GameRule>,
    /// A list of experiments that are either enabled or disabled in the world that the player
    /// spawns in.
    #[len_type(u32)]
    pub experiments: Vec<ExperimentData>,
    /// Specifies if any experiments were previously toggled in this world. It is likely used for
    /// metrics.
    pub experiments_previously_toggled: bool,
    /// Specifies if the world had the bonus map setting enabled when generating it. It is unclear
    /// why this is sent to the client, but it does not have any effect client-side either way.
    pub bonus_chest_enabled: bool,
    /// Specifies if the world has the start with map setting enabled, meaning each joining player
    /// obtains a map. This should always be set to false, because the client obtains a map all on
    /// its own accord if this is set to true.
    pub start_with_map_enabled: bool,
    /// The permission level of the player. This is used to determine what actions the player can
    /// perform.
    #[enum_header(VarI32)]
    pub player_permissions: PermissionLevel,
    /// The radius around the player in which chunks are ticked. Most servers set this value to a
    /// fixed number, as it does not necessarily affect anything client-side.
    pub server_chunk_tick_radius: i32,
    /// Specifies if the behaviour pack of the world is locked, meaning it cannot be disabled from
    /// the world. This is typically set for worlds on the marketplace that have a dedicated
    /// behaviour pack.
    pub has_locked_behaviour_pack: bool,
    /// Specifies if the texture pack of the world is locked, meaning it cannot be disabled from the
    /// world. This is typically set for worlds on the marketplace that have a dedicated texture
    /// pack.
    pub has_locked_texture_pack: bool,
    /// Specifies if the world from the server was from a locked world template. For servers, this
    /// should always be set to false.
    pub from_locked_world_template: bool,
    /// It is not exactly clear what this field does.
    pub msa_gamer_tags_only: bool,
    /// Specifies if the world was from a world template. For servers, this should always be set to
    /// false.
    pub from_world_template: bool,
    /// Specifies if the world was a template that locks all settings that change properties above
    /// in the settings GUI. Servers that do not allow things such as setting game rules through the
    /// GUI should set this to true.
    pub world_template_settings_locked: bool,
    /// A hack that Mojang put in place to preserve backwards compatibility with old villagers. The
    /// bool is never actually read though, so it has no functionality.
    pub only_spawn_v1_villagers: bool,
    /// Specifies if persona skins are disabled for the current game session.
    pub persona_disabled: bool,
    /// Specifies if custom skins are disabled for the current game session.
    pub custom_skins_disabled: bool,
    /// Specifies if players will be sent a chat message when using certain emotes.
    pub emote_chat_muted: bool,
    /// The version of the game from which Vanilla features will be used. The exact function this
    /// field isn't clear.
    pub base_game_version: String,
    /// The width of the world if the world is a limited world. For unlimited worlds, this is set
    /// to zero.
    pub limited_world_width: i32,
    /// The height of the world if the world is a limited world. For unlimited worlds, this is set
    /// to zero.
    pub limited_world_depth: i32,
    /// Specifies if the server runs with the new nether introduced in the 1.16 update.
    pub new_nether: bool,
    /// Education Edition feature that transmits education resource settings to clients.
    pub education_shared_resource_uri: EducationSharedResourceURI,
    /// Specifies if experimental gameplay should be force enabled. For servers, this should always
    /// be set to false.
    pub force_experimental_gameplay: bool,
    /// Specifies the level of restriction on in-game chat.
    pub chat_restriction_level: ChatRestrictionLevel,
    /// Specifies if the client should ignore other players when interacting with the world.
    pub disable_player_interactions: bool,
    /// Base64 encoded world ID that is used to identify the world.
    pub level_id: String,
    /// The name of the world that the player is joining. Note that this field shows up above the
    /// player list for the rest of the game session, and cannot be changed. Setting the server name
    /// to this field is recommended.
    pub world_name: String,
    /// UUID specific to the premium world template that might have been used to generate the world.
    /// Servers should always fill out an empty string for this.
    pub template_content_identity: String,
    /// Specifies if the world was a trial world, meaning features are limited and there is a time
    /// limit on the world.
    pub trial: bool,
    /// Specifies movement settings the server has enabled, which can regulate how movement data and
    /// block breaking data is sent and processed by the server and client.
    pub player_movement_settings: PlayerMovementSettings,
    /// The total time that has elapsed since the start of the world.
    pub time: i64,
    /// The seed used to seed the random used to produce enchantments in the enchantment table. Note
    /// that the exact correct random implementation must be used to produce the correct results
    /// both client-side and server-side.
    pub enchantment_seed: VarI32,
    /// A list of all custom blocks registered on the server.
    #[len_type(VarU32)]
    pub blocks: Vec<BlockEntry>,
    /// A list of all items with their legacy IDs which are available in the game. Failing to send
    /// any of the items that are in the game will crash mobile clients.
    #[len_type(VarU32)]
    pub items: Vec<ItemEntry>,
    /// A unique ID specifying the session of the player. A random UUID should be filled out for
    /// this field.
    pub multi_player_correlation_id: String,
    /// Specifies if the server authoritative inventory system is enabled. This is a new system
    /// introduced in 1.16. Backwards compatibility with the old system has, to some extent, been
    /// preserved, but will eventually be removed.
    pub server_authoritative_inventory: bool,
    /// The version of the game the server is running. This is likely used for telemetry.
    pub game_version: String,
    /// Contains properties that should be applied on the player. These properties are the same as
    /// the ones that are sent in the SyncActorProperty packet.
    pub property_data: NBT<NetworkLittleEndian>,
    /// Checksum to ensure block states between the server and client match. This can simply be left
    /// empty, and the client will avoid trying to verify it.
    pub server_block_state_checksum: u64,
    /// UUID that identifies the template that was used to generate the world. Servers that do not
    /// use a world based off of a template can set this to an empty UUID.
    pub world_template_id: Uuid,
    /// Specifies if the client should use the features registered in the FeatureRegistry packet to
    /// generate terrain client-side to save on bandwidth.
    pub client_side_generation: bool,
    /// Whether the client should use the hash of a block's name as its network ID rather than its
    /// index in the expected block palette. This is useful for servers that wish to support
    /// multiple protocol versions and custom blocks, but it will result in extra bytes being
    /// written for every block in a sub chunk palette.
    pub use_block_network_id_hashes: bool,
    /// The use for this field is currently unknown.
    pub server_authorative_sound: bool,
}

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum EditorWorldType {
    NotEditor,
    Project,
    TestLevel,
}
#[proto(i16)]
#[derive(Debug, Clone)]
pub enum SpawnBiomeType {
    Default,
    USerDefined,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum ChatRestrictionLevel {
    None,
    Dropped,
    Disabled,
}

#[proto(VarI32)]
#[derive(Debug, Copy, Clone)]
pub enum EducationEditionRegion {
    None,
    RestOfWorld,
    China,
}

#[proto(VarI32)]
#[derive(Debug, Clone)]
pub enum GamePublishSetting {
    None,
    InviteOnly,
    FriendsOnly,
    FriendsOfFriends,
    Public,
}
