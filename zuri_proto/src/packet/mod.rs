use crate::encodable_enum;

use change_mob_property::*;
use death_info::*;
use disconnect::*;
use resource_pack_client_response::*;
use resource_packs_info::*;
use resource_pack_stack::*;
use editor_network::*;
use feature_registry::*;
use game_test_request::*;
use game_test_results::*;
use lesson_progress::*;
use login::*;
use play_status::*;
use server_to_client_handshake::*;
use client_to_server_handshake::*;
use request_ability::*;
use request_network_settings::*;
use request_permissions::*;
use server_stats::*;
use toast_request::*;
use update_abilities::*;
use update_adventure_settings::*;
use update_client_input_locks::*;
use agent_action::*;
use dimension_data::*;
use text::*;
use ticking_area_load_status::*;
use code_builder_source::*;
use set_time::*;
use script_message::*;
use client_start_item_cooldown::*;
use start_game::*;
use sub_chunk_request::*;
use sub_chunk::*;
use photo_info_request::*;
use add_player::*;
use update_sub_chunk_blocks::*;
use create_photo::*;
use add_actor::*;
use education_resource_uri::*;
use npc_dialogue::*;
use remove_actor::*;
use simulation_type::*;
use remove_volume_entity::*;
use add_item_actor::*;
use add_volume_entity::*;
use take_item_actor::*;
use client_bound_debug_renderer::*;
use move_actor_absolute::*;
use filter_text::*;
use item_component::*;
use move_player::*;
use correct_player_move_prediction::*;
use player_fog::*;
use camera_shake::*;
use passenger_jump::*;
use animate_entity::*;
use update_block::*;
use motion_prediction_hints::*;
use add_painting::*;
use packet_violation_warning::*;
use debug_info::*;
use tick_sync::*;
use position_tracking_db_client_request::*;
use level_event::*;
use position_tracking_db_server_broadcast::*;
use emote_list::*;
use block_event::*;
use update_player_game_type::*;
use code_builder::*;
use actor_event::*;
use player_armour_damage::*;
use item_stack_response::*;
use item_stack_request::*;
use mob_effect::*;
use player_enchant_options::*;
use creative_content::*;
use player_auth_input::*;
use update_attributes::*;
use network_settings::*;
use completed_using_item::*;
use anvil_damage::*;
use inventory_transaction::*;
use settings_command::*;
use multi_player_settings::*;
use emote::*;
use education_settings::*;
use client_cache_miss_response::*;
use client_cache_blob_status::*;
use structure_template_data_response::*;
use structure_template_data_request::*;
use on_screen_texture_animation::*;
use map_create_locked_copy::*;
use client_cache_status::*;
use remove_entity::*;
use mob_equipment::*;
use lectern_update::*;
use level_event_generic::*;
use level_sound_event::*;
use biome_definition_list::*;
use mob_armour_equipment::*;
use network_chunk_publisher_update::*;
use available_actor_identifiers::*;
use interact::*;
use spawn_particle_effect::*;
use script_custom_event::*;
use block_pick_request::*;
use network_stack_latency::*;
use update_soft_enum::*;
use actor_pick_request::*;
use set_local_player_as_initialised::*;
use set_scoreboard_identity::*;
use move_actor_delta::*;
use player_action::*;
use update_block_synced::*;
use lab_table::*;
use hurt_armour::*;
use set_score::*;
use set_display_objective::*;
use remove_objective::*;
use set_actor_data::*;
use set_default_game_type::*;
use set_actor_motion::*;
use show_profile::*;
use server_settings_response::*;
use set_actor_link::*;
use server_settings_request::*;
use modal_form_response::*;
use set_health::*;
use modal_form_request::*;
use photo_transfer::*;
use set_spawn_position::*;
use npc_request::*;
use book_edit::*;
use animate::*;
use set_last_hurt_by::*;
use automation_client_connect::*;
use sub_client_login::*;
use player_skin::*;
use purchase_receipt::*;
use show_store_offer::*;
use structure_block_update::*;
use add_behaviour_tree::*;
use set_title::*;
use stop_sound::*;
use play_sound::*;
use transfer::*;
use resource_pack_chunk_request::*;
use resource_pack_chunk_data::*;
use respawn::*;
use resource_pack_data_info::*;
use container_open::*;
use update_equip::*;
use update_trade::*;
use container_close::*;
use command_output::*;
use command_block_update::*;
use player_hot_bar::*;
use inventory_content::*;
use command_request::*;
use inventory_slot::*;
use available_commands::*;
use show_credits::*;
use boss_event::*;
use camera::*;
use crafting_data::*;
use game_rules_changed::*;
use item_frame_drop_item::*;
use crafting_event::*;
use chunk_radius_updated::*;
use request_chunk_radius::*;
use map_info_request::*;
use gui_data_pick_item::*;
use adventure_settings::*;
use client_bound_map_item_data::*;
use spawn_experience_orb::*;
use event::*;
use block_actor_data::*;
use simple_event::*;
use player_input::*;
use player_list::*;
use set_player_game_type::*;
use level_chunk::*;
use change_dimension::*;
use set_commands_enabled::*;
use set_difficulty::*;


pub mod change_mob_property;
pub mod death_info;
pub mod disconnect;
pub mod resource_pack_client_response;
pub mod resource_packs_info;
pub mod resource_pack_stack;
pub mod editor_network;
pub mod feature_registry;
pub mod game_test_request;
pub mod game_test_results;
pub mod lesson_progress;
pub mod login;
pub mod play_status;
pub mod server_to_client_handshake;
pub mod client_to_server_handshake;
pub mod request_ability;
pub mod request_network_settings;
pub mod request_permissions;
pub mod server_stats;
pub mod toast_request;
pub mod update_abilities;
pub mod update_adventure_settings;
pub mod update_client_input_locks;
pub mod agent_action;
pub mod dimension_data;
pub mod text;
pub mod ticking_area_load_status;
pub mod code_builder_source;
pub mod set_time;
pub mod script_message;
pub mod client_start_item_cooldown;
pub mod start_game;
pub mod sub_chunk_request;
pub mod sub_chunk;
pub mod photo_info_request;
pub mod add_player;
pub mod update_sub_chunk_blocks;
pub mod create_photo;
pub mod add_actor;
pub mod education_resource_uri;
pub mod npc_dialogue;
pub mod remove_actor;
pub mod simulation_type;
pub mod remove_volume_entity;
pub mod add_item_actor;
pub mod add_volume_entity;
pub mod take_item_actor;
pub mod client_bound_debug_renderer;
pub mod move_actor_absolute;
pub mod filter_text;
pub mod item_component;
pub mod move_player;
pub mod correct_player_move_prediction;
pub mod player_fog;
pub mod camera_shake;
pub mod passenger_jump;
pub mod animate_entity;
pub mod update_block;
pub mod motion_prediction_hints;
pub mod add_painting;
pub mod packet_violation_warning;
pub mod debug_info;
pub mod tick_sync;
pub mod position_tracking_db_client_request;
pub mod level_event;
pub mod position_tracking_db_server_broadcast;
pub mod emote_list;
pub mod block_event;
pub mod update_player_game_type;
pub mod code_builder;
pub mod actor_event;
pub mod player_armour_damage;
pub mod item_stack_response;
pub mod item_stack_request;
pub mod mob_effect;
pub mod player_enchant_options;
pub mod creative_content;
pub mod player_auth_input;
pub mod update_attributes;
pub mod network_settings;
pub mod completed_using_item;
pub mod anvil_damage;
pub mod inventory_transaction;
pub mod settings_command;
pub mod multi_player_settings;
pub mod emote;
pub mod education_settings;
pub mod client_cache_miss_response;
pub mod client_cache_blob_status;
pub mod structure_template_data_response;
pub mod structure_template_data_request;
pub mod on_screen_texture_animation;
pub mod map_create_locked_copy;
pub mod client_cache_status;
pub mod remove_entity;
pub mod mob_equipment;
pub mod lectern_update;
pub mod level_event_generic;
pub mod level_sound_event;
pub mod biome_definition_list;
pub mod mob_armour_equipment;
pub mod network_chunk_publisher_update;
pub mod available_actor_identifiers;
pub mod interact;
pub mod spawn_particle_effect;
pub mod script_custom_event;
pub mod block_pick_request;
pub mod network_stack_latency;
pub mod update_soft_enum;
pub mod actor_pick_request;
pub mod set_local_player_as_initialised;
pub mod set_scoreboard_identity;
pub mod move_actor_delta;
pub mod player_action;
pub mod update_block_synced;
pub mod lab_table;
pub mod hurt_armour;
pub mod set_score;
pub mod set_display_objective;
pub mod remove_objective;
pub mod set_actor_data;
pub mod set_default_game_type;
pub mod set_actor_motion;
pub mod show_profile;
pub mod server_settings_response;
pub mod set_actor_link;
pub mod server_settings_request;
pub mod modal_form_response;
pub mod set_health;
pub mod modal_form_request;
pub mod photo_transfer;
pub mod set_spawn_position;
pub mod npc_request;
pub mod book_edit;
pub mod animate;
pub mod set_last_hurt_by;
pub mod automation_client_connect;
pub mod sub_client_login;
pub mod player_skin;
pub mod purchase_receipt;
pub mod show_store_offer;
pub mod structure_block_update;
pub mod add_behaviour_tree;
pub mod set_title;
pub mod stop_sound;
pub mod play_sound;
pub mod transfer;
pub mod resource_pack_chunk_request;
pub mod resource_pack_chunk_data;
pub mod respawn;
pub mod resource_pack_data_info;
pub mod container_open;
pub mod update_equip;
pub mod update_trade;
pub mod container_close;
pub mod command_output;
pub mod command_block_update;
pub mod player_hot_bar;
pub mod inventory_content;
pub mod command_request;
pub mod inventory_slot;
pub mod available_commands;
pub mod show_credits;
pub mod boss_event;
pub mod camera;
pub mod crafting_data;
pub mod game_rules_changed;
pub mod item_frame_drop_item;
pub mod crafting_event;
pub mod chunk_radius_updated;
pub mod request_chunk_radius;
pub mod map_info_request;
pub mod gui_data_pick_item;
pub mod adventure_settings;
pub mod client_bound_map_item_data;
pub mod spawn_experience_orb;
pub mod event;
pub mod block_actor_data;
pub mod simple_event;
pub mod player_input;
pub mod player_list;
pub mod set_player_game_type;
pub mod level_chunk;
pub mod change_dimension;
pub mod set_commands_enabled;
pub mod set_difficulty;
pub mod add_entity;
mod container_set_data;

encodable_enum!(
        pub enum Packets {
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

trait Packet {
    fn write(&self, writer: &mut Writer);
    fn read(reader: &mut Reader) -> Self;
}
