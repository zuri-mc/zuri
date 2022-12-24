use crate::encodable_enum;

mod change_mob_property;
mod death_info;
mod disconnect;
mod resource_pack_client_response;
mod resource_packs_info;
mod resource_pack_stack;
mod editor_network;
mod feature_registry;
mod game_test_request;
mod game_test_results;
mod lesson_progress;
mod login;
mod play_status;
mod server_to_client_handshake;
mod client_to_server_handshake;
mod request_ability;
mod request_network_settings;
mod request_permissions;
mod server_stats;
mod toast_request;
mod update_abilities;
mod update_adventure_settings;
mod update_client_input_locks;
mod agent_action;
mod dimension_data;
mod text;
mod ticking_area_load_status;
mod code_builder_source;
mod set_time;
mod script_message;
mod client_start_item_cooldown;
mod start_game;
mod sub_chunk_request;
mod sub_chunk;
mod photo_info_request;
mod add_player;
mod update_sub_chunk_blocks;
mod create_photo;
mod add_actor;
mod education_resource_uri;
mod npc_dialogue;
mod remove_actor;
mod simulation_type;
mod remove_volume_entity;
mod add_item_actor;
mod add_volume_entity;
mod take_item_actor;
mod client_bound_debug_renderer;
mod move_actor_absolute;
mod filter_text;
mod item_component;
mod move_player;
mod correct_player_move_prediction;
mod player_fog;
mod camera_shake;
mod passenger_jump;
mod animate_entity;
mod update_block;
mod motion_prediction_hints;
mod add_painting;
mod packet_violation_warning;
mod debug_info;
mod tick_sync;
mod position_tracking_db_client_request;
mod level_event;
mod position_tracking_db_server_broadcast;
mod emote_list;
mod block_event;
mod update_player_game_type;
mod code_builder;
mod actor_event;
mod player_armour_damage;
mod item_stack_response;
mod item_stack_request;
mod mob_effect;
mod player_enchant_options;
mod creative_content;
mod player_auth_input;
mod update_attributes;
mod network_settings;
mod completed_using_item;
mod anvil_damage;
mod inventory_transaction;
mod settings_command;
mod multi_player_settings;
mod emote;
mod education_settings;
mod client_cache_miss_response;
mod client_cache_blob_status;
mod structure_template_data_response;
mod structure_template_data_request;
mod on_screen_texture_animation;
mod map_create_locked_copy;
mod client_cache_status;
mod remove_entity;
mod mob_equipment;
mod lectern_update;
mod level_event_generic;
mod level_sound_event;
mod biome_definition_list;
mod mob_armour_equipment;
mod network_chunk_publisher_update;
mod available_actor_identifiers;
mod interact;
mod spawn_particle_effect;
mod script_custom_event;
mod block_pick_request;
mod network_stack_latency;
mod update_soft_enum;
mod actor_pick_request;
mod set_local_player_as_initialised;
mod set_scoreboard_identity;
mod move_actor_delta;
mod player_action;
mod update_block_synced;
mod lab_table;
mod hurt_armour;
mod set_score;
mod set_display_objective;
mod remove_objective;
mod set_actor_data;
mod set_default_game_type;
mod set_actor_motion;
mod show_profile;
mod server_settings_response;
mod set_actor_link;
mod server_settings_request;
mod modal_form_response;
mod set_health;
mod modal_form_request;
mod photo_transfer;
mod set_spawn_position;
mod npc_request;
mod book_edit;
mod animate;
mod set_last_hurt_by;
mod automation_client_connect;
mod sub_client_login;
mod player_skin;
mod purchase_receipt;
mod show_store_offer;
mod structure_block_update;
mod add_behaviour_tree;
mod set_title;
mod stop_sound;
mod play_sound;
mod transfer;
mod resource_pack_chunk_request;
mod resource_pack_chunk_data;
mod respawn;
mod resource_pack_data_info;
mod container_open;
mod update_equip;
mod update_trade;
mod container_close;
mod command_output;
mod command_block_update;
mod player_hot_bar;
mod inventory_content;
mod command_request;
mod inventory_slot;
mod available_commands;
mod show_credits;
mod boss_event;
mod camera;
mod crafting_data;
mod game_rules_changed;
mod item_frmae_drop_item;
mod crafting_event;
mod chunk_radius_updated;
mod request_chunk_radius;
mod map_info_request;
mod gui_data_pick_item;
mod adventure_settings;
mod client_bound_map_item_data;
mod spawn_experience_orb;
mod event;
mod block_actor_data;
mod simple_event;
mod player_input;
mod player_list;
mod set_player_game_type;
mod level_chunk;
mod change_dimension;
mod set_commands_enabled;
mod set_difficulty;

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
