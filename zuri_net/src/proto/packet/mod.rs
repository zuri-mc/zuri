use crate::encodable_enum;
use crate::proto::io::{Reader, Writer};

use actor_event::*;
use actor_pick_request::*;
use add_actor::*;
use add_behaviour_tree::*;
use add_entity::*;
use add_item_actor::*;
use add_painting::*;
use add_player::*;
use add_volume_entity::*;
use adventure_settings::*;
use agent_action::*;
use animate::*;
use animate_entity::*;
use anvil_damage::*;
use automation_client_connect::*;
use available_actor_identifiers::*;
use available_commands::*;
use biome_definition_list::*;
use block_actor_data::*;
use block_event::*;
use block_pick_request::*;
use book_edit::*;
use boss_event::*;
use camera::*;
use camera_shake::*;
use change_dimension::*;
use change_mob_property::*;
use chunk_radius_updated::*;
use client_bound_debug_renderer::*;
use client_bound_map_item_data::*;
use client_cache_blob_status::*;
use client_cache_miss_response::*;
use client_cache_status::*;
use client_start_item_cooldown::*;
use client_to_server_handshake::*;
use code_builder::*;
use code_builder_source::*;
use command_block_update::*;
use command_output::*;
use command_request::*;
use completed_using_item::*;
use container_close::*;
use container_open::*;
use container_set_data::*;
use correct_player_move_prediction::*;
use crafting_data::*;
use crafting_event::*;
use create_photo::*;
use creative_content::*;
use death_info::*;
use debug_info::*;
use dimension_data::*;
use disconnect::*;
use editor_network::*;
use education_resource_uri::*;
use education_settings::*;
use emote::*;
use emote_list::*;
use event::*;
use feature_registry::*;
use filter_text::*;
use game_rules_changed::*;
use game_test_request::*;
use game_test_results::*;
use gui_data_pick_item::*;
use hurt_armour::*;
use interact::*;
use inventory_content::*;
use inventory_slot::*;
use inventory_transaction::*;
use item_component::*;
use item_frame_drop_item::*;
use item_stack_request::*;
use item_stack_response::*;
use lab_table::*;
use lectern_update::*;
use lesson_progress::*;
use level_chunk::*;
use level_event::*;
use level_event_generic::*;
use level_sound_event::*;
use login::*;
use map_create_locked_copy::*;
use map_info_request::*;
use mob_armour_equipment::*;
use mob_effect::*;
use mob_equipment::*;
use modal_form_request::*;
use modal_form_response::*;
use motion_prediction_hints::*;
use move_actor_absolute::*;
use move_actor_delta::*;
use move_player::*;
use multi_player_settings::*;
use network_chunk_publisher_update::*;
use network_settings::*;
use network_stack_latency::*;
use npc_dialogue::*;
use npc_request::*;
use on_screen_texture_animation::*;
use packet_violation_warning::*;
use passenger_jump::*;
use photo_info_request::*;
use photo_transfer::*;
use play_sound::*;
use play_status::*;
use player_action::*;
use player_armour_damage::*;
use player_auth_input::*;
use player_enchant_options::*;
use player_fog::*;
use player_hot_bar::*;
use player_input::*;
use player_list::*;
use player_skin::*;
use position_tracking_db_client_request::*;
use position_tracking_db_server_broadcast::*;
use purchase_receipt::*;
use remove_actor::*;
use remove_entity::*;
use remove_objective::*;
use remove_volume_entity::*;
use request_ability::*;
use request_chunk_radius::*;
use request_network_settings::*;
use request_permissions::*;
use resource_pack_chunk_data::*;
use resource_pack_chunk_request::*;
use resource_pack_client_response::*;
use resource_pack_data_info::*;
use resource_pack_stack::*;
use resource_packs_info::*;
use respawn::*;
use script_custom_event::*;
use script_message::*;
use server_settings_request::*;
use server_settings_response::*;
use server_stats::*;
use server_to_client_handshake::*;
use set_actor_data::*;
use set_actor_link::*;
use set_actor_motion::*;
use set_commands_enabled::*;
use set_default_game_type::*;
use set_difficulty::*;
use set_display_objective::*;
use set_health::*;
use set_last_hurt_by::*;
use set_local_player_as_initialised::*;
use set_player_game_type::*;
use set_score::*;
use set_scoreboard_identity::*;
use set_spawn_position::*;
use set_time::*;
use set_title::*;
use settings_command::*;
use show_credits::*;
use show_profile::*;
use show_store_offer::*;
use simple_event::*;
use simulation_type::*;
use spawn_experience_orb::*;
use spawn_particle_effect::*;
use start_game::*;
use stop_sound::*;
use structure_block_update::*;
use structure_template_data_request::*;
use structure_template_data_response::*;
use sub_chunk::*;
use sub_chunk_request::*;
use sub_client_login::*;
use sync_actor_property::*;
use take_item_actor::*;
use text::*;
use tick_sync::*;
use ticking_area_load_status::*;
use toast_request::*;
use transfer::*;
use update_abilities::*;
use update_adventure_settings::*;
use update_attributes::*;
use update_block::*;
use update_block_synced::*;
use update_client_input_locks::*;
use update_equip::*;
use update_player_game_type::*;
use update_soft_enum::*;
use update_sub_chunk_blocks::*;
use update_trade::*;

pub mod actor_event;
pub mod actor_pick_request;
pub mod add_actor;
pub mod add_behaviour_tree;
pub mod add_entity;
pub mod add_item_actor;
pub mod add_painting;
pub mod add_player;
pub mod add_volume_entity;
pub mod adventure_settings;
pub mod agent_action;
pub mod animate;
pub mod animate_entity;
pub mod anvil_damage;
pub mod automation_client_connect;
pub mod available_actor_identifiers;
pub mod available_commands;
pub mod biome_definition_list;
pub mod block_actor_data;
pub mod block_event;
pub mod block_pick_request;
pub mod book_edit;
pub mod boss_event;
pub mod camera;
pub mod camera_shake;
pub mod change_dimension;
pub mod change_mob_property;
pub mod chunk_radius_updated;
pub mod client_bound_debug_renderer;
pub mod client_bound_map_item_data;
pub mod client_cache_blob_status;
pub mod client_cache_miss_response;
pub mod client_cache_status;
pub mod client_start_item_cooldown;
pub mod client_to_server_handshake;
pub mod code_builder;
pub mod code_builder_source;
pub mod command_block_update;
pub mod command_output;
pub mod command_request;
pub mod completed_using_item;
pub mod container_close;
pub mod container_open;
pub mod container_set_data;
pub mod correct_player_move_prediction;
pub mod crafting_data;
pub mod crafting_event;
pub mod create_photo;
pub mod creative_content;
pub mod death_info;
pub mod debug_info;
pub mod dimension_data;
pub mod disconnect;
pub mod editor_network;
pub mod education_resource_uri;
pub mod education_settings;
pub mod emote;
pub mod emote_list;
pub mod event;
pub mod feature_registry;
pub mod filter_text;
pub mod game_rules_changed;
pub mod game_test_request;
pub mod game_test_results;
pub mod gui_data_pick_item;
pub mod hurt_armour;
pub mod interact;
pub mod inventory_content;
pub mod inventory_slot;
pub mod inventory_transaction;
pub mod item_component;
pub mod item_frame_drop_item;
pub mod item_stack_request;
pub mod item_stack_response;
pub mod lab_table;
pub mod lectern_update;
pub mod lesson_progress;
pub mod level_chunk;
pub mod level_event;
pub mod level_event_generic;
pub mod level_sound_event;
pub mod login;
pub mod map_create_locked_copy;
pub mod map_info_request;
pub mod mob_armour_equipment;
pub mod mob_effect;
pub mod mob_equipment;
pub mod modal_form_request;
pub mod modal_form_response;
pub mod motion_prediction_hints;
pub mod move_actor_absolute;
pub mod move_actor_delta;
pub mod move_player;
pub mod multi_player_settings;
pub mod network_chunk_publisher_update;
pub mod network_settings;
pub mod network_stack_latency;
pub mod npc_dialogue;
pub mod npc_request;
pub mod on_screen_texture_animation;
pub mod packet_violation_warning;
pub mod passenger_jump;
pub mod photo_info_request;
pub mod photo_transfer;
pub mod play_sound;
pub mod play_status;
pub mod player_action;
pub mod player_armour_damage;
pub mod player_auth_input;
pub mod player_enchant_options;
pub mod player_fog;
pub mod player_hot_bar;
pub mod player_input;
pub mod player_list;
pub mod player_skin;
pub mod position_tracking_db_client_request;
pub mod position_tracking_db_server_broadcast;
pub mod purchase_receipt;
pub mod remove_actor;
pub mod remove_entity;
pub mod remove_objective;
pub mod remove_volume_entity;
pub mod request_ability;
pub mod request_chunk_radius;
pub mod request_network_settings;
pub mod request_permissions;
pub mod resource_pack_chunk_data;
pub mod resource_pack_chunk_request;
pub mod resource_pack_client_response;
pub mod resource_pack_data_info;
pub mod resource_pack_stack;
pub mod resource_packs_info;
pub mod respawn;
pub mod script_custom_event;
pub mod script_message;
pub mod server_settings_request;
pub mod server_settings_response;
pub mod server_stats;
pub mod server_to_client_handshake;
pub mod set_actor_data;
pub mod set_actor_link;
pub mod set_actor_motion;
pub mod set_commands_enabled;
pub mod set_default_game_type;
pub mod set_difficulty;
pub mod set_display_objective;
pub mod set_health;
pub mod set_last_hurt_by;
pub mod set_local_player_as_initialised;
pub mod set_player_game_type;
pub mod set_score;
pub mod set_scoreboard_identity;
pub mod set_spawn_position;
pub mod set_time;
pub mod set_title;
pub mod settings_command;
pub mod show_credits;
pub mod show_profile;
pub mod show_store_offer;
pub mod simple_event;
pub mod simulation_type;
pub mod spawn_experience_orb;
pub mod spawn_particle_effect;
pub mod start_game;
pub mod stop_sound;
pub mod structure_block_update;
pub mod structure_template_data_request;
pub mod structure_template_data_response;
pub mod sub_chunk;
pub mod sub_chunk_request;
pub mod sub_client_login;
pub mod sync_actor_property;
pub mod take_item_actor;
pub mod text;
pub mod tick_sync;
pub mod ticking_area_load_status;
pub mod toast_request;
pub mod transfer;
pub mod update_abilities;
pub mod update_adventure_settings;
pub mod update_attributes;
pub mod update_block;
pub mod update_block_synced;
pub mod update_client_input_locks;
pub mod update_equip;
pub mod update_player_game_type;
pub mod update_soft_enum;
pub mod update_sub_chunk_blocks;
pub mod update_trade;

encodable_enum!(
        #[derive(Debug, Clone)]
        pub enum Packet {
            Login = 1,
            PlayStatus = 2,
            ServerToClientHandshake = 3,
            ClientToServerHandshake = 4,
            Disconnect = 5,
            ResourcePacksInfo = 6,
            ResourcePackStack = 7,
            ResourcePackClientResponse = 8,
            Text = 9,
            SetTime = 10,
            StartGame = 11,
            AddPlayer = 12,
            AddActor = 13,
            RemoveActor = 14,
            AddItemActor = 15,
            TakeItemActor = 17,
            MoveActorAbsolute = 18,
            MovePlayer = 19,
            PassengerJump = 20,
            UpdateBlock = 21,
            AddPainting = 22,
            TickSync = 23,
            LevelEvent = 25,
            BlockEvent = 26,
            ActorEvent = 27,
            MobEffect = 28,
            UpdateAttributes = 29,
            InventoryTransaction = 30,
            MobEquipment = 31,
            MobArmourEquipment = 32,
            Interact = 33,
            BlockPickRequest = 34,
            ActorPickRequest = 35,
            PlayerAction = 36,
            HurtArmour = 38,
            SetActorData = 39,
            SetActorMotion = 40,
            SetActorLink = 41,
            SetHealth = 42,
            SetSpawnPosition = 43,
            Animate = 44,
            Respawn = 45,
            ContainerOpen = 46,
            ContainerClose = 47,
            PlayerHotBar = 48,
            InventoryContent = 49,
            InventorySlot = 50,
            ContainerSetData = 51,
            CraftingData = 52,
            CraftingEvent = 53,
            GUIDataPickItem = 54,
            AdventureSettings = 55,
            BlockActorData = 56,
            PlayerInput = 57,
            LevelChunk = 58,
            SetCommandsEnabled = 59,
            SetDifficulty = 60,
            ChangeDimension = 61,
            SetPlayerGameType = 62,
            PlayerList = 63,
            SimpleEvent = 64,
            Event = 65,
            SpawnExperienceOrb = 66,
            ClientBoundMapItemData = 67,
            MapInfoRequest = 68,
            RequestChunkRadius = 69,
            ChunkRadiusUpdated = 70,
            ItemFrameDropItem = 71,
            GameRulesChanged = 72,
            Camera = 73,
            BossEvent = 74,
            ShowCredits = 75,
            AvailableCommands = 76,
            CommandRequest = 77,
            CommandBlockUpdate = 78,
            CommandOutput = 79,
            UpdateTrade = 80,
            UpdateEquip = 81,
            ResourcePackDataInfo = 82,
            ResourcePackChunkData = 83,
            ResourcePackChunkRequest = 84,
            Transfer = 85,
            PlaySound = 86,
            StopSound = 87,
            SetTitle = 88,
            AddBehaviourTree = 89,
            StructureBlockUpdate = 90,
            ShowStoreOffer = 91,
            PurchaseReceipt = 92,
            PlayerSkin = 93,
            SubClientLogin = 94,
            AutomationClientConnect = 95,
            SetLastHurtBy = 96,
            BookEdit = 97,
            NPCRequest = 98,
            PhotoTransfer = 99,
            ModalFormRequest = 100,
            ModalFormResponse = 101,
            ServerSettingsRequest = 102,
            ServerSettingsResponse = 103,
            ShowProfile = 104,
            SetDefaultGameType = 105,
            RemoveObjective = 106,
            SetDisplayObjective = 107,
            SetScore = 108,
            LabTable = 109,
            UpdateBlockSynced = 110,
            MoveActorDelta = 111,
            SetScoreboardIdentity = 112,
            SetLocalPlayerAsInitialised = 113,
            UpdateSoftEnum = 114,
            NetworkStackLatency = 115,
            ScriptCustomEvent = 117,
            SpawnParticleEffect = 118,
            AvailableActorIdentifiers = 119,
            NetworkChunkPublisherUpdate = 121,
            BiomeDefinitionList = 122,
            LevelSoundEvent = 123,
            LevelEventGeneric = 124,
            LecternUpdate = 125,
            AddEntity = 127,
            RemoveEntity = 128,
            ClientCacheStatus = 129,
            MapCreateLockedCopy = 130,
            OnScreenTextureAnimation = 131,
            StructureTemplateDataRequest = 132,
            StructureTemplateDataResponse = 133,
            ClientCacheBlobStatus = 135,
            ClientCacheMissResponse = 136,
            EducationSettings = 137,
            Emote = 138,
            MultiPlayerSettings = 139,
            SettingsCommand = 140,
            AnvilDamage = 141,
            CompletedUsingItem = 142,
            NetworkSettings = 143,
            PlayerAuthInput = 144,
            CreativeContent = 145,
            PlayerEnchantOptions = 146,
            ItemStackRequest = 147,
            ItemStackResponse = 148,
            PlayerArmourDamage = 149,
            CodeBuilder = 150,
            UpdatePlayerGameType = 151,
            EmoteList = 152,
            PositionTrackingDBServerBroadcast = 153,
            PositionTrackingDBClientRequest = 154,
            DebugInfo = 155,
            PacketViolationWarning = 156,
            MotionPredictionHints = 157,
            AnimateEntity = 158,
            CameraShake = 159,
            PlayerFog = 160,
            CorrectPlayerMovePrediction = 161,
            ItemComponent = 162,
            FilterText = 163,
            ClientBoundDebugRenderer = 164,
            SyncActorProperty = 165,
            AddVolumeEntity = 166,
            RemoveVolumeEntity = 167,
            SimulationType = 168,
            NPCDialogue = 169,
            EducationResourceURI = 170,
            CreatePhoto = 171,
            UpdateSubChunkBlocks = 172,
            PhotoInfoRequest = 173,
            SubChunk = 174,
            SubChunkRequest = 175,
            ClientStartItemCooldown = 176,
            ScriptMessage = 177,
            CodeBuilderSource = 178,
            TickingAreasLoadStatus = 179,
            DimensionData = 180,
            AgentAction = 181,
            ChangeMobProperty = 182,
            LessonProgress = 183,
            RequestAbility = 184,
            RequestPermissions = 185,
            ToastRequest = 186,
            UpdateAbilities = 187,
            UpdateAdventureSettings = 188,
            DeathInfo = 189,
            EditorNetwork = 190,
            FeatureRegistry = 191,
            ServerStats = 192,
            RequestNetworkSettings = 193,
            GameTestRequest = 194,
            GameTestResults = 195,
            UpdateClientInputLocks = 196,
        }
);

trait PacketType {
    fn write(&self, writer: &mut Writer);
    fn read(reader: &mut Reader) -> Self;
}
