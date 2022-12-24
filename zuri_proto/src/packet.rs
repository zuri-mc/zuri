use std::collections::BTreeMap;
use bytes::Bytes;
use glam::{IVec2, Vec2, Vec3};
use uuid::Uuid;
use crate::data::*;
use crate::enums::*;
use crate::io::{Reader, Writer};

macro_rules! packets {
    (
            $(#[$attr:meta])*
            $vis:vis enum $name:ident {
                $($elem:ident = $discrim:literal$(,)?)+
            }
    ) => {
        $(#[$attr])*
        #[repr(u32)]
        $vis enum $name {
            $($elem($elem) = $discrim,)+
        }

        impl $name {
            $vis fn read(reader: &mut Reader) -> Self {
                return match reader.var_u32() & 0x3FF {
                    $($discrim => $name::$elem($elem::read(reader)),)+
                    _ => panic!("unknown packet type"),
                }
            }

            $vis fn write(&self, writer: &mut Writer) {
                match self {
                    $($name::$elem(pk) => {
                        writer.var_u32($discrim);
                        pk.write(writer);
                    },)+
                    _ => panic!("unknown packet type"),
                }
            }
        }

        /// Allow the packets to be converted to the enum with Into.
        $(impl Into<$name> for $elem {
            fn into(self) -> $name {
                $name::$elem(self)
            }
        })+
    };
}

packets!(
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

/// Sent when the client initially tries to join the server. It is the first packet sent and contains information
/// specific to the player.
#[derive(Debug)]
pub struct Login {
    /// The protocol version of the player. The player is disconnected if the protocol is incompatible with the
    /// protocol of the server. It has been superseded by the protocol version sent in the RequestNetworkSettings
    /// packet, so this should no longer be used by the server.
    pub client_protocol: i32,
    /// A string containing information about the player and JWTs that may be used to verify if the player is connected
    /// to XBOX Live. The connection request also contains the necessary client public key to initiate encryption.
    pub connection_request: Bytes,
}

impl Packet for Login {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.client_protocol);
        writer.byte_slice(&self.connection_request);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_protocol: reader.i32_be(),
            connection_request: reader.byte_slice(),
        }
    }
}

/// Sent by the server to update a player on the play status. This includes failed statuses due to a mismatched version,
/// but also success statuses.
#[derive(Debug)]
pub struct PlayStatus {
    /// The status of the packet.
    pub status: PlayStatusType,
}

impl Packet for PlayStatus {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(num::ToPrimitive::to_i32(&self.status).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self { status: num::FromPrimitive::from_i32(reader.i32_be()).unwrap() }
    }
}

/// Sent by the server to the client to complete the key exchange in order to initialise encryption on client and server
/// side. It is followed up by a ClientToServerHandshake packet from the client.
#[derive(Debug)]
pub struct ServerToClientHandshake {
    /// A raw JWT token containing data such as the public key from the server, the algorithm used and the server's
    /// token. It is used for the client to produce a shared secret.
    pub jwt: Bytes,
}

impl Packet for ServerToClientHandshake {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.jwt);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { jwt: reader.byte_slice() }
    }
}

/// Sent by the client in response to a ServerToClientHandshake packet sent by the server. It is the first encrypted
/// packet in the login handshake and serves as a confirmation that encryption is correctly initialised client side.
#[derive(Debug)]
pub struct ClientToServerHandshake {}

impl Packet for ClientToServerHandshake {
    fn write(&self, _: &mut Writer) {}

    fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

/// Sent by the server to disconnect the client using an optional message to send as the disconnect screen.
#[derive(Debug)]
pub struct Disconnect {
    /// An optional message to show when disconnected. If left empty, the disconnection screen will be hidden.
    pub message: Option<String>,
}

impl Packet for Disconnect {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.message.is_some());
        if self.message.is_some() {
            writer.string(self.message.unwrap().as_str());
        }
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            message: if reader.bool() { None } else { Some(reader.string()) },
        }
    }
}

/// Sent by the server to inform the client on what resource packs the server has. It sends a list of the resource packs
/// it has and basic information on them like the version and description.
#[derive(Debug)]
pub struct ResourcePacksInfo {
    /// Specifies if the client must accept the texture packs the server has in order to join the server. If set to
    /// true, the client gets the option to either download the resource packs and join, or quit entirely. Behaviour
    /// packs never have to be downloaded.
    pub texture_pack_required: bool,
    /// Specifies if any of the resource packs contain scripts in them. If set to true, only clients that support
    /// scripts will be able to download them.
    pub has_scripts: bool,
    /// A list of behaviour packs that the client needs to download before joining the server. All of these behaviour
    /// packs will be applied together.
    pub behaviour_packs: Vec<BehaviourPackInfo>,
    /// A list of texture packs that the client needs to download before joining the server. The order of these texture
    /// packs is not relevant in this packet. It is however important in the ResourcePackStack packet.
    pub texture_packs: Vec<TexturePackInfo>,
    /// The use of this field is currently unknown.
    pub forcing_server_packs: bool,
}

impl Packet for ResourcePacksInfo {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.texture_pack_required);
        writer.bool(self.has_scripts);

        writer.u16(self.behaviour_packs.len() as u16);
        self.behaviour_packs.iter().for_each(|pack| pack.write(writer));

        writer.u16(self.texture_packs.len() as u16);
        self.texture_packs.iter().for_each(|pack| pack.write(writer));

        writer.bool(self.forcing_server_packs);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            texture_pack_required: reader.bool(),
            has_scripts: reader.bool(),
            behaviour_packs: (0..reader.u16()).map(|_| BehaviourPackInfo::read(reader)).collect(),
            texture_packs: (0..reader.u16()).map(|_| TexturePackInfo::read(reader)).collect(),
            forcing_server_packs: reader.bool(),
        }
    }
}

/// Sent by the server to send the order in which resource packs and behaviour packs should be applied (and downloaded)
/// by the client.
#[derive(Debug)]
pub struct ResourcePackStack {
    /// Specifies if the client must accept the texture packs the server has in order to join the server. If set to
    /// true, the client gets the option to either download the resource packs and join, or quit entirely. Behaviour
    /// packs never have to be downloaded.
    pub texture_pack_required: bool,
    /// A list of behaviour packs that the client needs to download before joining the server. All of these behaviour
    /// packs will be applied together, and the order does not necessarily matter.
    pub behaviour_packs: Vec<StackResourcePack>,
    /// A list of texture packs that the client needs to download before joining the server. The order of these texture
    /// packs specifies the order that they are applied in on the client side. The first in the list will be applied
    /// before the rest.
    pub texture_packs: Vec<StackResourcePack>,
    /// The vanilla version that the client should set its resource pack stack to.
    pub base_game_version: String,
    /// A list of experiments that are either enabled or disabled in the world that the player spawns in. It is not
    /// clear why experiments are sent both here and in the StartGame packet.
    pub experiments: Vec<ExperimentData>,
    /// Specifies if any experiments were previously toggled in this world. It is probably used for metrics.
    pub experiments_previously_toggled: bool,
}

impl Packet for ResourcePackStack {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.texture_pack_required);

        writer.var_u32(self.behaviour_packs.len() as u32);
        self.behaviour_packs.iter().for_each(|pack| pack.write(writer));

        writer.var_u32(self.texture_packs.len() as u32);
        self.texture_packs.iter().for_each(|pack| pack.write(writer));

        writer.string(self.base_game_version.as_str());

        writer.u32(self.experiments.len() as u32);
        self.experiments.iter().for_each(|experiment| experiment.write(writer));

        writer.bool(self.experiments_previously_toggled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            texture_pack_required: reader.bool(),
            behaviour_packs: (0..reader.var_u32()).map(|_| StackResourcePack::read(reader)).collect(),
            texture_packs: (0..reader.var_u32()).map(|_| StackResourcePack::read(reader)).collect(),
            base_game_version: reader.string(),
            experiments: (0..reader.u32()).map(|_| ExperimentData::read(reader)).collect(),
            experiments_previously_toggled: reader.bool(),
        }
    }
}

/// Sent by the client in response to resource packets sent by the server. It is used to let the server know what action
/// needs to be taken for the client to have all resource packs ready and set.
#[derive(Debug)]
pub struct ResourcePackClientResponse {
    /// The response type the client gave.
    pub response: ResourcePackResponse,
    /// A list of resource pack UUIDs combined with their version that need to be downloaded, if the Response field is
    /// PackResponseSendPacks.
    pub packs_to_download: Vec<String>,
}

impl Packet for ResourcePackClientResponse {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.response).unwrap());

        writer.u16(self.packs_to_download.len() as u16);
        self.packs_to_download.iter().for_each(|pack| writer.string(pack.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            response: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            packs_to_download: (0..reader.u16()).map(|_| reader.string()).collect(),
        }
    }
}

/// Sent by the client to the server to send chat messages, and by the server to the client to forward or send messages,
/// which may be chat, popups, tips etc.
#[derive(Debug)]
pub struct Text {
    /// The type of the text sent. When a client sends this to the server, it should always be Chat.
    pub text_type: TextType,
    /// Specifies if any of the messages need to be translated. It seems that where % is found in translatable text
    /// types, these are translated regardless of this bool. Translatable text types include Translation, Tip, Popup,
    /// and JukeboxPopup.
    pub needs_translation: bool,
    /// The name of the source of the messages. This source is displayed in text types such as Chat and Whisper, where
    /// typically the username is shown.
    pub source_name: String,
    /// The message of the packet. This field is set for each TextType and is the main component of the packet.
    pub message: String,
    /// A list of parameters that should be filled into the message. These parameters are only written if the type of
    /// the packet is Translation, Tip, Popup or JukeboxPopup.
    pub parameters: Vec<String>,
    /// The XBOX Live user ID of the player that sent the message. It is only set for packets of text type Chat. When
    /// sent to a player, the player will only be shown the chat message if a player with this XUID is present in the
    /// player list and not muted, or if the XUID is empty.
    pub xuid: String,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo Switch). It is
    /// otherwise an empty string, and is used to decide which players are able to chat with each other.
    pub platform_chat_id: String,
}

impl Packet for Text {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.text_type).unwrap());
        writer.bool(self.needs_translation);
        match self.text_type {
            TextType::Chat | TextType::Whisper | TextType::Announcement => {
                writer.string(self.source_name.as_str());
                writer.string(self.message.as_str());
            }
            TextType::Raw | TextType::Tip | TextType::System | TextType::Object | TextType::ObjectWhisper | TextType::ObjectAnnouncement => {
                writer.string(self.message.as_str());
            }
            TextType::Translation | TextType::Popup | TextType::JukeboxPopup => {
                writer.string(self.message.as_str());
                writer.var_u32(self.parameters.len() as u32);
                self.parameters.iter().for_each(|parameter| writer.string(parameter.as_str()));
            }
        }
        writer.string(self.xuid.as_str());
        writer.string(self.platform_chat_id.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        let text_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            text_type,
            needs_translation: reader.bool(),
            source_name: if text_type == TextType::Chat || text_type == TextType::Whisper || text_type == TextType::Announcement {
                reader.string()
            } else {
                "".to_string()
            },
            message: reader.string(),
            parameters: if text_type == TextType::Translation || text_type == TextType::Popup || text_type == TextType::JukeboxPopup {
                (0..reader.var_u32()).map(|_| reader.string()).collect()
            } else {
                Vec::new()
            },
            xuid: reader.string(),
            platform_chat_id: reader.string(),
        }
    }
}

/// Sent by the server to update the current time client-side. The client actually advances time client-side by itself,
/// so this packet does not need to be sent each tick. It is a means of synchronising time between server and client.
#[derive(Debug)]
pub struct SetTime {
    /// The current time. The time is not limited to 24000 (time of day), but continues progressing after that.
    pub time: i32,
}

impl Packet for SetTime {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.time);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            time: reader.var_i32(),
        }
    }
}

/// Sent by the server to send information about the world the player will be spawned in. It contains information about
/// the position the player spawns in, and information about the world in general such as its game rules.
#[derive(Debug)]
pub struct StartGame {
    /// The unique ID of the player. The unique ID is a value that remains consistent across different sessions of the
    /// same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The game mode of the player. If set to Default, then the client will fall back to the game mode set in the
    /// world_game_mode field, which is the default game mode of the world.
    pub player_game_mode: GameType,
    /// The spawn position of the player in the world. In servers this is often the same as the world's spawn position
    /// found below. Otherwise, this is usually the position the player was last at when they left the world.
    pub player_position: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of 0. Pitch is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The seed used to generate the world.
    pub world_seed: i64,
    /// Specifies if the biome that the player spawns in is user defined (through behaviour packs) or builtin.
    pub spawn_biome_type: SpawnBiomeType,
    /// A readable name of the biome that the player spawned in, such as 'plains'. This might be a custom biome name
    /// if any custom biomes are present through behaviour packs.
    pub user_defined_biome_name: String,
    /// The dimension that the player spawns in. Most mini-game servers use the Overworld dimension.
    pub dimension: Dimension,
    /// The generator used for the world. Most vanilla worlds use the Overworld generator.
    pub generator: Generator,
    /// The game mode that a player gets when it first spawns in the world. It is shown in the settings and is used if
    /// the player_game_mode is set to Default.
    pub world_game_mode: GameType,
    /// The difficulty of the world. It is not exactly clear why this is sent to the client, as the client does not
    /// need to know the difficulty of the world.
    pub difficulty: Difficulty,
    /// The block on which the world spawn of the world. This coordinate has no effect on the place that the client
    /// spawns, but it does have an effect on the direction that a compass points.
    pub world_spawn: BlockPos,
    /// Specifies if achievements are disabled in the world. The client crashes if this value is set to true while the
    /// player's or the world's game mode is creative. It's recommended to simply always set this to false as a server.
    pub achievements_disabled: bool,
    /// Dictates if the world is in editor mode, a special mode recently introduced adding "powerful tools for editing
    /// worlds, intended for experienced creators."
    pub editor_world: bool,
    /// The time at which the day cycle was locked if the day cycle is disabled using the respective game rule. The
    /// client will maintain this time as long as the day cycle is disabled.
    pub day_cycle_lock_time: i32,
    /// Minecraft: Education Edition field that specifies what 'region' the world is in.
    pub education_edition_offer: EducationEditionRegion,
    /// Specifies if the world has Education Edition features enabled.
    pub education_features_enabled: bool,
    /// UUID used to identify the Education Edition server instance. It is usually unique.
    pub education_product_id: String,
    /// The intensity of the rain falling. When set to 0, no rain falls at all.
    pub rain_level: f32,
    /// The intensity of the thunder. This may actually be set independently from the rain level, meaning dark clouds
    /// can be produced without rain.
    pub lightning_level: f32,
    /// Specifies if the world has platform locked content.
    pub confirmed_platform_locked_content: bool,
    /// Specifies if the world is a multi-player game. This should always be set to true for servers.
    pub multi_player_game: bool,
    /// Specifies if LAN broadcast was intended to be enabled for the world.
    pub lan_broadcast_enabled: bool,
    /// The mode used to broadcast the joined game across XBOX Live.
    pub xbl_broadcast_mode: GamePublishSetting,
    /// The mode used to broadcast the joined game across the platform.
    pub platform_broadcast_mode: GamePublishSetting,
    /// Specifies if commands are enabled for the player. It is recommended to always set this to true on a server, as
    /// setting it to false means the player cannot, under any circumstance, use a command.
    pub commands_enabled: bool,
    /// Specifies if the texture pack the world might hold is required, meaning the client was forced to download it
    /// before joining. This is usually set to true for servers.
    pub texture_pack_required: bool,
    /// The game rules currently active with their respective values. The value of these game rules may be either
    /// 'bool', 'int32' or 'float32'. Some game rules are server side only and don't need to be sent to the client.
    pub game_rules: Vec<GameRule>,
    /// A list of experiments that are either enabled or disabled in the world that the player spawns in.
    pub experiments: Vec<ExperimentData>,
    /// Specifies if any experiments were previously toggled in this world. It is likely used for metrics.
    pub experiments_previously_toggled: bool,
    /// Specifies if the world had the bonus map setting enabled when generating it. It is unclear why this is sent to
    /// the client, but it does not have any effect client-side either way.
    pub bonus_chest_enabled: bool,
    /// Specifies if the world has the start with map setting enabled, meaning each joining player obtains a map. This
    /// should always be set to false, because the client obtains a map all on its own accord if this is set to true.
    pub start_with_map_enabled: bool,
    /// The permission level of the player. This is used to determine what actions the player can perform.
    pub player_permissions: PermissionLevel,
    /// The radius around the player in which chunks are ticked. Most servers set this value to a fixed number, as it
    /// does not necessarily affect anything client-side.
    pub server_chunk_tick_radius: i32,
    /// Specifies if the behaviour pack of the world is locked, meaning it cannot be disabled from the world. This is
    /// typically set for worlds on the marketplace that have a dedicated behaviour pack.
    pub has_locked_behaviour_pack: bool,
    /// Specifies if the texture pack of the world is locked, meaning it cannot be disabled from the world. This is
    /// typically set for worlds on the marketplace that have a dedicated texture pack.
    pub has_locked_texture_pack: bool,
    /// Specifies if the world from the server was from a locked world template. For servers, this should always be set
    /// to false.
    pub from_locked_world_template: bool,
    /// It is not exactly clear what this field does.
    pub msa_gamer_tags_only: bool,
    /// Specifies if the world was from a world template. For servers, this should always be set to false.
    pub from_world_template: bool,
    /// Specifies if the world was a template that locks all settings that change properties above in the settings GUI.
    /// Servers that do not allow things such as setting game rules through the GUI should set this to true.
    pub world_template_settings_locked: bool,
    /// A hack that Mojang put in place to preserve backwards compatibility with old villagers. The bool is never
    /// actually read though, so it has no functionality.
    pub only_spawn_v1_villagers: bool,
    /// Specifies if persona skins are disabled for the current game session.
    pub persona_disabled: bool,
    /// Specifies if custom skins are disabled for the current game session.
    pub custom_skins_disabled: bool,
    /// The version of the game from which Vanilla features will be used. The exact function this field isn't clear.
    pub base_game_version: String,
    /// The width of the world if the world is a limited world. For unlimited worlds, this is set to 0.
    pub limited_world_width: i32,
    /// The height of the world if the world is a limited world. For unlimited worlds, this is set to 0.
    pub limited_world_depth: i32,
    /// Specifies if the server runs with the new nether introduced in the 1.16 update.
    pub new_nether: bool,
    /// Education Edition feature that transmits education resource settings to clients.
    pub education_shared_resource_uri: EducationSharedResourceURI,
    /// Specifies if experimental gameplay should be force enabled. For servers, this should always be set to false.
    pub force_experimental_gameplay: Option<bool>,
    /// Base64 encoded world ID that is used to identify the world.
    pub level_id: String,
    /// The name of the world that the player is joining. Note that this field shows up above the player list for the
    /// rest of the game session, and cannot be changed. Setting the server name to this field is recommended.
    pub world_name: String,
    /// UUID specific to the premium world template that might have been used to generate the world. Servers should
    /// always fill out an empty string for this.
    pub template_content_identity: String,
    /// Specifies if the world was a trial world, meaning features are limited and there is a time limit on the world.
    pub trial: bool,
    /// Specifies movement settings the server has enabled, which can regulate how movement data and block breaking data
    /// is sent and processed by the server and client.
    pub player_movement_settings: PlayerMovementSettings,
    /// The total time that has elapsed since the start of the world.
    pub time: i64,
    /// The seed used to seed the random used to produce enchantments in the enchantment table. Note that the exact
    /// correct random implementation must be used to produce the correct results both client-side and server-side.
    pub enchantment_seed: i32,
    /// A list of all custom blocks registered on the server.
    pub blocks: Vec<BlockEntry>,
    /// A list of all items with their legacy IDs which are available in the game. Failing to send any of the items that
    /// are in the game will crash mobile clients.
    pub items: Vec<ItemEntry>,
    /// A unique ID specifying the session of the player. A random UUID should be filled out for this field.
    pub multi_player_correlation_id: String,
    /// Specifies if the server authoritative inventory system is enabled. This is a new system introduced in 1.16.
    /// Backwards compatibility with the old system has, to some extent, been preserved, but will eventually be removed.
    pub server_authoritative_inventory: bool,
    /// The version of the game the server is running. This is likely used for telemetry.
    pub game_version: String,
    /// Contains properties that should be applied on the player. These properties are the same as the ones that are
    /// sent in the SyncActorProperty packet.
    //TODO: NBT
    // pub property_data: dyn Any,
    /// Checksum to ensure block states between the server and client match. This can simply be left empty, and the
    /// client will avoid trying to verify it.
    pub server_block_state_checksum: u64,
    /// Specifies if the client should use the features registered in the FeatureRegistry packet to generate terrain
    /// client-side to save on bandwidth.
    pub client_side_generation: bool,
    /// UUID that identifies the template that was used to generate the world. Servers that do not use a world based
    /// off of a template can set this to an empty UUID.
    pub world_template_id: Uuid,
    /// Specifies the level of restriction on in-game chat.
    pub chat_restriction_level: ChatRestrictionLevel,
    /// Specifies if the client should ignore other players when interacting with the world.
    pub disable_player_interactions: bool,
}

impl Packet for StartGame {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);

        writer.var_i32(num::ToPrimitive::to_i32(&self.player_game_mode).unwrap());
        writer.vec3(self.player_position);

        writer.f32(self.pitch);
        writer.f32(self.yaw);

        writer.i64(self.world_seed);

        writer.i16(num::ToPrimitive::to_i16(&self.spawn_biome_type).unwrap());
        writer.string(self.user_defined_biome_name.as_str());

        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.generator).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.world_game_mode).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.difficulty).unwrap());

        writer.u_block_pos(self.world_spawn);
        writer.bool(self.achievements_disabled);
        writer.bool(self.editor_world);
        writer.var_i32(self.day_cycle_lock_time);

        writer.var_i32(num::ToPrimitive::to_i32(&self.education_edition_offer).unwrap());
        writer.bool(self.education_features_enabled);
        writer.string(self.education_product_id.as_str());

        writer.f32(self.rain_level);
        writer.f32(self.lightning_level);

        writer.bool(self.confirmed_platform_locked_content);
        writer.bool(self.multi_player_game);
        writer.bool(self.lan_broadcast_enabled);

        writer.var_i32(num::ToPrimitive::to_i32(&self.xbl_broadcast_mode).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.platform_broadcast_mode).unwrap());

        writer.bool(self.commands_enabled);
        writer.bool(self.texture_pack_required);

        writer.var_u32(self.game_rules.len() as u32);
        self.game_rules.iter().for_each(|game_rule| game_rule.write(writer));
        writer.u32(self.experiments.len() as u32);
        self.experiments.iter().for_each(|experiment| experiment.write(writer));

        writer.bool(self.experiments_previously_toggled);
        writer.bool(self.bonus_chest_enabled);
        writer.bool(self.start_with_map_enabled);

        writer.var_i32(num::ToPrimitive::to_i32(&self.player_permissions).unwrap());

        writer.i32(self.server_chunk_tick_radius);

        writer.bool(self.has_locked_behaviour_pack);
        writer.bool(self.has_locked_texture_pack);
        writer.bool(self.from_locked_world_template);
        writer.bool(self.msa_gamer_tags_only);
        writer.bool(self.from_world_template);
        writer.bool(self.world_template_settings_locked);
        writer.bool(self.only_spawn_v1_villagers);
        writer.bool(self.persona_disabled);
        writer.bool(self.custom_skins_disabled);

        writer.string(self.base_game_version.as_str());

        writer.i32(self.limited_world_width);
        writer.i32(self.limited_world_depth);

        writer.bool(self.new_nether);

        self.education_shared_resource_uri.write(writer);

        writer.optional(&self.force_experimental_gameplay, writer.bool);

        writer.u8(num::ToPrimitive::to_u8(&self.chat_restriction_level).unwrap());
        writer.bool(self.disable_player_interactions);

        writer.string(self.level_id.as_str());
        writer.string(self.world_name.as_str());
        writer.string(self.template_content_identity.as_str());

        writer.bool(self.trial);

        self.player_movement_settings.write(writer);

        writer.i64(self.time);

        writer.var_i32(self.enchantment_seed);

        writer.var_u32(self.blocks.len() as u32);
        self.blocks.iter().for_each(|entry| entry.write(writer));

        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|entry| entry.write(writer));

        writer.string(self.multi_player_correlation_id.as_str());

        writer.bool(self.server_authoritative_inventory);

        writer.string(self.game_version.as_str());

        // TODO: w.NBT(&pk.PropertyData, nbt.NetworkLittleEndian)

        writer.u64(self.server_block_state_checksum);

        writer.uuid(self.world_template_id);
        writer.bool(self.client_side_generation);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),

            player_game_mode: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            player_position: reader.vec3(),

            pitch: reader.f32(),
            yaw: reader.f32(),

            world_seed: reader.i64(),

            spawn_biome_type: num::FromPrimitive::from_i16(reader.i16()).unwrap(),
            user_defined_biome_name: reader.string(),

            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            generator: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            world_game_mode: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            difficulty: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),

            world_spawn: reader.u_block_pos(),
            achievements_disabled: reader.bool(),
            editor_world: reader.bool(),
            day_cycle_lock_time: reader.var_i32(),

            education_edition_offer: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            education_features_enabled: reader.bool(),
            education_product_id: reader.string(),

            rain_level: reader.f32(),
            lightning_level: reader.f32(),

            confirmed_platform_locked_content: reader.bool(),
            multi_player_game: reader.bool(),
            lan_broadcast_enabled: reader.bool(),

            xbl_broadcast_mode: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            platform_broadcast_mode: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),

            commands_enabled: reader.bool(),
            texture_pack_required: reader.bool(),

            game_rules: (0..reader.var_u32()).map(|_| GameRule::read(reader)).collect(),

            experiments: (0..reader.u32()).map(|_| ExperimentData::read(reader)).collect(),

            experiments_previously_toggled: reader.bool(),
            bonus_chest_enabled: reader.bool(),
            start_with_map_enabled: reader.bool(),

            player_permissions: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),

            server_chunk_tick_radius: reader.i32(),

            has_locked_behaviour_pack: reader.bool(),
            has_locked_texture_pack: reader.bool(),
            from_locked_world_template: reader.bool(),
            msa_gamer_tags_only: reader.bool(),
            from_world_template: reader.bool(),
            world_template_settings_locked: reader.bool(),
            only_spawn_v1_villagers: reader.bool(),
            persona_disabled: reader.bool(),
            custom_skins_disabled: reader.bool(),

            base_game_version: reader.string(),

            limited_world_width: reader.i32(),
            limited_world_depth: reader.i32(),

            new_nether: reader.bool(),

            education_shared_resource_uri: EducationSharedResourceURI::read(reader),

            force_experimental_gameplay: reader.optional(reader.bool),

            chat_restriction_level: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            disable_player_interactions: reader.bool(),

            level_id: reader.string(),
            world_name: reader.string(),
            template_content_identity: reader.string(),

            trial: reader.bool(),

            player_movement_settings: PlayerMovementSettings::read(reader),

            time: reader.i64(),

            enchantment_seed: reader.var_i32(),

            blocks: (0..reader.var_u32()).map(|_| BlockEntry::read(reader)).collect(),

            items: (0..reader.var_u32()).map(|_| ItemEntry::read(reader)).collect(),

            multi_player_correlation_id: reader.string(),

            server_authoritative_inventory: reader.bool(),

            game_version: reader.string(),

            // property_data: {
            //     // TODO: NBT
            // },

            server_block_state_checksum: reader.u64(),

            world_template_id: reader.uuid(),
            client_side_generation: reader.bool(),
        }
    }
}

/// Sent by the server to the client to make a player entity show up client-side. It is one of the few entities that
/// cannot be sent using the AddActor packet.
#[derive(Debug)]
pub struct AddPlayer {
    /// The UUID of the player. It is the same UUID that the client sent in the Login packet at the start of the
    /// session. A player with this UUID must exist in the player list (built up using the PlayerList packet), for it to
    /// show up in-game.
    pub uuid: Uuid,
    /// The name of the player. This username is the username that will be set as the initial name tag of the player.
    pub username: String,
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// An identifier only set for particular platforms when chatting (presumably only for Nintendo Switch). It is
    /// otherwise an empty string, and is used to decide which players are able to chat with each other.
    pub platform_chat_id: String,
    /// The position to spawn the player on. If the player is on a distance that the viewer cannot see it, the player
    /// will still show up if the viewer moves closer.
    pub position: Vec3,
    /// The initial velocity the player spawns with. This velocity will initiate client side movement of the player.
    pub velocity: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of 0. Pitch is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the player. A different value for head_yaw
    /// than Yaw means that the player will have its head turned.
    pub head_yaw: f32,
    /// The item that the player is holding. The item is shown to the viewer as soon as the player itself shows up.
    /// Needless to say that this field is rather pointless, as more packets still must be sent for armour to show up.
    pub held_item: ItemInstance,
    /// The game type of the player. If set to Spectator, the player will not be shown to viewers.
    pub game_type: GameType,
    /// A map of entity metadata, which includes flags and data properties that alter in particular the way the player
    /// looks. Flags include ones such as 'on fire' and 'sprinting'. The meta values are indexed by their property key.
    // TODO: Implement entity metadata.
    // pub entity_metadata: dyn Any,
    /// A list of properties that the entity inhibits. These properties define specific attributes of the entity.
    // TODO: Implement entity properties.
    // pub entity_properties: dyn Any,
    /// Represents various data about the abilities of a player, such as ability layers or permissions.
    pub ability_data: AbilityData,
    /// A list of entity links that are currently active on the player. These links alter the way the player shows up
    /// when first spawned in terms of it shown as riding an entity. Setting these links is important for new viewers
    /// to see the player is riding another entity.
    pub entity_links: Vec<EntityLink>,
    /// The device ID set in one of the files found in the storage of the device of the player. It may be changed
    /// freely, so it should not be relied on for anything.
    pub device_id: String,
    /// The build platform/device OS of the player that is about to be added, as sent in the Login packet.
    pub build_platform: i32, // TODO: Use DeviceOS enum
}

impl Packet for AddPlayer {
    fn write(&self, writer: &mut Writer) {
        writer.uuid(self.uuid);
        writer.string(self.username.as_str());
        writer.var_u64(self.entity_runtime_id);
        writer.string(self.platform_chat_id.as_str());

        writer.vec3(self.position);
        writer.vec3(self.velocity);

        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.f32(self.head_yaw);

        self.held_item.write(writer);

        writer.var_i32(num::ToPrimitive::to_i32(&self.game_type).unwrap());
        // TODO: Entity metadata.
        // TODO: Entity properties.
        self.ability_data.write(writer);

        writer.var_u32(self.entity_links.len() as u32);
        self.entity_links.iter().for_each(|link| link.write(writer));

        writer.string(self.device_id.as_str());
        writer.i32(self.build_platform);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.uuid(),
            username: reader.string(),
            entity_runtime_id: reader.var_u64(),
            platform_chat_id: reader.string(),

            position: reader.vec3(),
            velocity: reader.vec3(),

            pitch: reader.f32(),
            yaw: reader.f32(),
            head_yaw: reader.f32(),

            held_item: ItemInstance::read(reader),

            game_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            // entity_metadata: {
            //     // TODO: Entity metadata.
            // },
            // entity_properties: {
            //     // TODO: Entity properties.
            // },
            ability_data: AbilityData::read(reader),

            entity_links: (0..reader.var_u32()).map(|_| EntityLink::read(reader)).collect(),

            device_id: reader.string(),
            build_platform: reader.i32(),
        }
    }
}

/// Sent by the server to the client to spawn an entity to the player. It is used for every entity except other players,
/// for which the AddPlayer packet is used.
#[derive(Debug)]
pub struct AddActor {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across different sessions of the
    /// same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The string entity type of the entity. A list of these entities may be found online.
    pub entity_type: String,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot see it, the entity
    /// will still show up if the player moves closer.
    pub position: Vec3,
    /// The initial velocity the entity spawns with. This velocity will initiate client side movement of the entity.
    pub velocity: Vec3,
    /// The vertical rotation of the entity. Facing straight forward yields a pitch of 0. Pitch is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the entity. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the entity. A different value for head_yaw
    /// than yaw means that the entity will have its head turned.
    pub head_yaw: f32,
    /// The same as yaw, except that it applies specifically to the body of the entity. A different value for body_yaw
    /// than head_yaw means that the entity will have its body turned, although it is unclear what the difference
    /// between body_yaw and yaw is.
    pub body_yaw: f32,
    /// A slice of attributes that the entity has. It includes attributes such as its health, movement speed, etc.
    pub attributes: Vec<AttributeValue>,
    /// A map of entity metadata, which includes flags and data properties that alter in particular the way the entity
    /// looks. Flags include ones such as 'on fire' and 'sprinting'. The meta values are indexed by their property key.
    // TODO: Implement entity metadata.
    // pub entity_metadata: dyn Any,
    /// A list of properties that the entity inhibits. These properties define specific attributes of the entity.
    // TODO: Implement entity properties.
    // pub entity_properties: dyn Any,
    /// A list of entity links that are currently active on the entity. These links alter the way the entity shows up
    /// when first spawned in terms of it shown as riding an entity. Setting these links is important for new viewers
    /// to see the entity is riding another entity.
    pub entity_links: Vec<EntityLink>,
}

impl Packet for AddActor {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);
        writer.string(self.entity_type.as_str());

        writer.vec3(self.position);
        writer.vec3(self.velocity);

        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.f32(self.head_yaw);
        writer.f32(self.body_yaw);

        writer.var_u32(self.attributes.len() as u32);
        self.attributes.iter().for_each(|attribute| attribute.write(writer));

        // TODO: Entity metadata.
        // TODO: Entity properties.

        writer.var_u32(self.entity_links.len() as u32);
        self.entity_links.iter().for_each(|link| link.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),
            entity_type: reader.string(),

            position: reader.vec3(),
            velocity: reader.vec3(),

            pitch: reader.f32(),
            yaw: reader.f32(),
            head_yaw: reader.f32(),
            body_yaw: reader.f32(),

            attributes: (0..reader.var_u32()).map(|_| AttributeValue::read(reader)).collect(),

            // entity_metadata: {
            //     // TODO: Entity metadata.
            // },
            // entity_properties: {
            //     // TODO: Entity properties.
            // },

            entity_links: (0..reader.var_u32()).map(|_| EntityLink::read(reader)).collect(),
        }
    }
}

/// Sent by the server to remove an entity that currently exists in the world from the client-side. Sending this packet
/// if the client cannot already see this entity will have no effect.
#[derive(Debug)]
pub struct RemoveActor {
    /// The unique ID of the entity to be removed. The unique ID is a value that remains consistent across different
    /// sessions of the same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
}

impl Packet for RemoveActor {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_unique_id: reader.i64() }
    }
}

/// Sent by the server to the client to make an item entity show up. It is one of the few entities that cannot be sent
/// using the AddActor packet
#[derive(Debug)]
pub struct AddItemActor {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across different sessions of the
    /// same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The item that is spawned. It must have a valid ID for it to show up client-side. If it is not a valid item,
    /// the client will crash when coming near.
    pub item: ItemInstance,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot see it, the entity
    /// will still show up if the player moves closer.
    pub position: Vec3,
    /// The initial velocity the entity spawns with. This velocity will initiate client side movement of the entity.
    pub velocity: Vec3,
    /// A map of entity metadata, which includes flags and data properties that alter in particular the way the entity
    /// looks. Flags include ones such as 'on fire' and 'sprinting'. The meta values are indexed by their property key.
    // TODO: Implement entity metadata.
    // pub entity_metadata: dyn Any,
    /// Specifies if the item was obtained by fishing it up using a fishing rod. It is not clear why the client needs
    /// to know this.
    pub from_fishing: bool,
}

impl Packet for AddItemActor {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);

        self.item.write(writer);

        writer.vec3(self.position);
        writer.vec3(self.velocity);

        // TODO: Entity metadata.
        writer.bool(self.from_fishing);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),

            item: ItemInstance::read(reader),

            position: reader.vec3(),
            velocity: reader.vec3(),

            // entity_metadata: {
            //     // TODO: Entity metadata.
            // },
            from_fishing: reader.bool(),
        }
    }
}

/// Sent by the server when a player picks up an item entity. It makes the item entity disappear to viewers and shows
/// the pick-up animation. The item entity is not actually removed from the world, but it is hidden from viewers.
#[derive(Debug)]
pub struct TakeItemActor {
    /// The entity runtime ID of the item that is being taken by another entity. It will disappear to viewers after
    /// showing the pick-up animation.
    pub item_entity_runtime_id: u64,
    /// The runtime ID of the entity that took the item, which is usually a player, but could be another entity like a
    /// zombie too.
    pub taker_entity_runtime_id: u64,
}

impl Packet for TakeItemActor {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.item_entity_runtime_id);
        writer.var_u64(self.taker_entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            item_entity_runtime_id: reader.var_u64(),
            taker_entity_runtime_id: reader.var_u64(),
        }
    }
}

/// Sent by the server to move an entity to an absolute position. It is typically used for movements where high accuracy
/// isn't needed, such as for long range teleporting.
#[derive(Debug)]
pub struct MoveActorAbsolute {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A combination of MoveFlags that specify details of the movement.
    pub flags: u8,
    /// The position to move the entity to. If the entity is on a distance that the player cannot see it, the entity
    /// will still show up if the player moves closer.
    pub position: Vec3,
    /// The rotation of the entity. The first value is the pitch, the second is the head yaw, and the third is the yaw.
    pub rotation: Vec3,
}

impl Packet for MoveActorAbsolute {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        writer.u8(self.flags);

        writer.vec3(self.position);
        writer.byte_f32(self.rotation.x);
        writer.byte_f32(self.rotation.y);
        writer.byte_f32(self.rotation.z);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),

            flags: reader.u8(),

            position: reader.vec3(),
            rotation: Vec3 {
                x: reader.byte_f32(),
                y: reader.byte_f32(),
                z: reader.byte_f32(),
            },
        }
    }
}

/// Sent by players to send their movement to the server, and by the server to update the movement of player entities
/// to other players. When using the new movement system, this is only sent by the server.
#[derive(Debug)]
pub struct MovePlayer {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The position to spawn the player on. If the player is on a distance that the viewer cannot see it, the player
    /// will still show up if the viewer moves closer.
    pub position: Vec3,
    /// The vertical rotation of the player. Facing straight forward yields a pitch of 0. Pitch is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the player. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the player. A different value for head_yaw
    /// than yaw means that the player will have its head turned.
    pub head_yaw: f32,
    /// The mode of the movement. It specifies the way the player's movement should be shown to other players.
    pub mode: MoveMode,
    /// Specifies if the player is considered on the ground. Note that proxies or hacked clients could fake this to
    /// always be true, so it should not be taken for granted.
    pub on_ground: bool,
    /// The runtime ID of the entity that the player might currently be riding. If not riding, this should be left zero.
    pub ridden_entity_runtime_id: u64,
    /// Written only if mode is Teleport. It specifies the cause of the teleportation.
    pub teleport_cause: TeleportCause,
    /// The entity type that caused the teleportation, for example, an ender pearl.
    pub teleport_source_entity_type: i32,
    /// The server tick at which the packet was sent. It is used in relation to CorrectPlayerMovePrediction.
    pub tick: u64,
}

impl Packet for MovePlayer {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        writer.vec3(self.position);
        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.f32(self.head_yaw);

        writer.u8(num::ToPrimitive::to_u8(&self.mode).unwrap());
        writer.bool(self.on_ground);
        writer.var_u64(self.ridden_entity_runtime_id);
        if self.mode == MoveMode::Teleport {
            writer.i32(num::ToPrimitive::to_i32(&self.teleport_cause).unwrap());
            writer.i32(self.teleport_source_entity_type);
        }

        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            entity_runtime_id: reader.var_u64(),
            position: reader.vec3(),
            pitch: reader.f32(),
            yaw: reader.f32(),
            head_yaw: reader.f32(),
            mode: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            on_ground: reader.bool(),
            ridden_entity_runtime_id: reader.var_u64(),
            teleport_cause: TeleportCause::None,
            teleport_source_entity_type: 0,
            tick: 0,
        };
        if packet.mode == MoveMode::Teleport {
            packet.teleport_cause = num::FromPrimitive::from_i32(reader.i32()).unwrap();
            packet.teleport_source_entity_type = reader.i32();
        }
        packet.tick = reader.var_u64();

        packet
    }
}

/// Sent by the client to the server when it jumps while riding an entity that has the WASDControlled entity flag set,
/// for example when riding a horse.
#[derive(Debug)]
pub struct PassengerJump {
    /// The strength of the jump, depending on how long the rider has held the jump button.
    pub jump_strength: i32,
}

impl Packet for PassengerJump {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.jump_strength);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { jump_strength: reader.var_i32() }
    }
}

/// Sent by the server to update a block client-side, without resending the entire chunk that the block is located in.
/// It is particularly useful for small modifications like block breaking/placing.
#[derive(Debug)]
pub struct UpdateBlock {
    /// The block position at which a block is updated.
    pub position: BlockPos,
    /// The runtime ID of the block that is placed at position after sending the packet to the client.
    pub new_block_runtime_id: u32,
    /// A combination of BlockUpdate flags that specify the way the block is updated client-side. Typically, sending
    /// only the Network flag is sufficient.
    pub flags: u32,
    /// The world layer on which the block is updated. For most blocks, this is the first layer, as that layer is the
    /// default layer to place blocks on, but for blocks inside of each other, this differs.
    pub layer: u32,
}

impl Packet for UpdateBlock {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_u32(self.new_block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u32(self.layer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            new_block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            layer: reader.var_u32(),
        }
    }
}

/// Sent by the server to the client to make a painting entity show up. It is one of the few entities that cannot be
/// sent using the AddActor packet.
#[derive(Debug)]
pub struct AddPainting {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across different sessions of the
    /// same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot see it, the entity
    /// will still show up if the player moves closer.
    pub position: Vec3,
    /// The facing direction of the painting.
    pub direction: i32,
    /// The title of the painting. It specifies the motive of the painting. The title of the painting must be valid.
    pub title: String,
}

impl Packet for AddPainting {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);

        writer.vec3(self.position);
        writer.var_i32(self.direction);
        writer.string(self.title.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),

            position: reader.vec3(),
            direction: reader.var_i32(),
            title: reader.string(),
        }
    }
}

/// Sent by the client and the server to maintain a synchronized, server-authoritative tick between the client and the
/// server. The client sends this packet first, and the server should reply with another one of these packets, including
/// the response time.
#[derive(Debug)]
pub struct TickSync {
    /// The timestamp on which the client sent this packet to the server. The server should fill out that same value
    /// when replying. The client_request_timestamp is always zero.
    pub client_request_timestamp: i64,
    /// The timestamp on which the server received the packet sent by the client. When the packet is sent by the client,
    /// this value is zero. server_reception_timestamp is generally the current tick of the server. It isn't an actual
    /// timestamp, as the field implies.
    pub server_reception_timestamp: i64,
}

impl Packet for TickSync {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.client_request_timestamp);
        writer.i64(self.server_reception_timestamp);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_request_timestamp: reader.i64(),
            server_reception_timestamp: reader.i64(),
        }
    }
}

/// Sent by the server to make a certain event in the level occur. It ranges from particles, to sounds, and other events
/// such as starting rain and block breaking.
#[derive(Debug)]
pub struct LevelEvent {
    /// The event that is being 'called'.
    pub event_type: LevelEventType,
    /// The position of the level event. Practically every event requires this Vec3 set for it, as particles, sounds and
    /// block editing relies on it.
    pub position: Vec3,
    /// An integer holding additional data of the event. The type of data held depends on the EventType.
    pub event_data: i32,
}

impl Packet for LevelEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.event_type).unwrap());
        writer.vec3(self.position);
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.vec3(),
            event_data: reader.var_i32(),
        }
    }
}

/// Sent by the server to initiate a certain event that has to do with blocks in specific, for example opening chests.
#[derive(Debug)]
pub struct BlockEvent {
    /// The position of the block that an event occurred at.
    pub position: BlockPos,
    /// The type of the block event. The event type decides the way the event data that follows is used.
    pub event_type: BlockEventType,
    /// Holds event type specific data. For chests, for example, opening the chest means the data must hold one, whereas
    /// closing it should hold zero.
    pub event_data: i32,
}

impl Packet for BlockEvent {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_i32(num::ToPrimitive::to_i32(&self.event_type).unwrap());
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            event_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            event_data: reader.var_i32(),
        }
    }
}

/// Sent by the server when a particular event happens that has to do with an entity. Some of these events are
/// entity-specific, for example a wolf shaking itself dry, but others are used for each entity, such as dying.
#[derive(Debug)]
pub struct ActorEvent {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The type of event to be called.
    pub event_type: ActorEventType,
    /// Optional data associated with a particular event. The data has a different function for different events,
    /// however most events don't use this field at all.
    pub event_data: i32,
}

impl Packet for ActorEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u8(num::ToPrimitive::to_u8(&self.event_type).unwrap());
        writer.var_i32(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            event_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            event_data: reader.var_i32(),
        }
    }
}

/// Sent by the server to apply an effect to the player, for example an effect like poison. It may also be used to
/// modify existing effects, or removing them completely.
#[derive(Debug)]
pub struct MobEffect {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The operation of the packet, specifying the result client-side.
    pub operation: MobEffectOperation,
    /// The type of the effect to be added, removed or modified.
    pub effect_type: MobEffectType,
    /// The amplifier of the effect. Take note that the amplifier is not the same as the effect's level. The level is
    /// usually one higher than the amplifier, and the amplifier can be negative to reverse the behaviour effect.
    pub amplifier: i32,
    /// Specifies if viewers of the entity that gets the effect shows particles around it. If set to false, no particles
    /// are emitted around the entity.
    pub particles: bool,
    /// The duration of the effect in seconds. After the duration has elapsed, the effect will be removed automatically
    /// client-side.
    pub duration: i32,
}

impl Packet for MobEffect {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u8(num::ToPrimitive::to_u8(&self.operation).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.effect_type).unwrap());
        writer.var_i32(self.amplifier);
        writer.bool(self.particles);
        writer.var_i32(self.duration);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            operation: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            effect_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            amplifier: reader.var_i32(),
            particles: reader.bool(),
            duration: reader.var_i32(),
        }
    }
}

/// Sent by the server to update an amount of attributes of any entity in the world. These attributes include ones such
/// as the health or the movement speed of the entity.
#[derive(Debug)]
pub struct UpdateAttributes {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A slice of new attributes that the entity gets. It includes attributes such as its health, movement speed, etc.
    /// Note that only changed attributes have to be sent in this packet. It is not required to send attributes that did
    /// not have their values changed.
    pub attributes: Vec<Attribute>,
    /// The server tick at which the packet was sent. It is used in relation to CorrectPlayerMovePrediction.
    pub tick: u64,
}

impl Packet for UpdateAttributes {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.var_u32(self.attributes.len() as u32);
        self.attributes.iter().for_each(|attribute| attribute.write(writer));
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            attributes: (0..reader.var_u32()).map(|_| Attribute::read(reader)).collect(),
            tick: reader.var_u64(),
        }
    }
}

/// Sent by the client. It essentially exists out of multiple sub-packets, each of which have something to do with the
/// inventory in one way or another. Some of these sub-packets directly relate to the inventory, others relate to
/// interaction with the world, that could potentially result in a change in the inventory.
#[derive(Debug)]
pub struct InventoryTransaction {
    /// ID that is only non-zero at times when sent by the client. The server should always send 0 for this. When this
    /// field is not zero, the legacy_set_item_slots slice below will have values in it. legacy_request_id ties in with
    /// the ItemStackResponse packet. If this field is non-zero, the server should respond with an ItemStackResponse
    /// packet. Some inventory actions such as dropping an item out of the hotbar are still one using this packet, and
    /// the ItemStackResponse packet needs to tie in with it.
    pub legacy_request_id: i32,
    /// Only present if the LegacyRequestID is non-zero. These item slots inform the server of the slots that were
    /// changed during the inventory transaction, and the server should send back an ItemStackResponse packet with these
    /// slots present in it. (Or false with no slots, if rejected.)
    pub legacy_set_item_slots: Vec<LegacySetItemSlot>,
    /// List of actions that took place, that form the inventory transaction together. Each of these actions hold one
    /// slot in which one item was changed to another. In general, the combination of all of these actions results in a
    /// balanced inventory transaction. This should be checked to ensure that no items are cheated into the inventory.
    pub actions: Vec<InventoryAction>,
    /// Data object that holds data specific to the type of transaction that the TransactionPacket held. Its concrete
    /// type must be one of Normal, Mismatch, UseItem, UseItemOnEntity or ReleaseItem. If empty, the transaction will be
    /// assumed to of type Normal.
    pub transaction_data: Box<dyn InventoryTransactionData>,
}

impl Packet for InventoryTransaction {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.legacy_request_id);
        if self.legacy_request_id != 0 {
            writer.var_u32(self.legacy_set_item_slots.len() as u32);
            self.legacy_set_item_slots.iter().for_each(|slot| slot.write(writer));
        }

        writer.var_u32(num::ToPrimitive::to_u32(&self.transaction_data.transaction_type()).unwrap());

        writer.var_u32(self.actions.len() as u32);
        self.actions.iter().for_each(|action| action.write(writer));

        self.transaction_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        let legacy_request_id = reader.var_i32();
        let legacy_set_item_slots = if legacy_request_id != 0 {
            (0..reader.var_u32()).map(|_| LegacySetItemSlot::read(reader)).collect()
        } else {
            Vec::new()
        };
        let transaction_type = num::FromPrimitive::from_u32(reader.var_u32()).unwrap();
        Self {
            legacy_request_id,
            legacy_set_item_slots,
            actions: (0..reader.var_u32()).map(|_| InventoryAction::read(reader)).collect(),
            transaction_data: match transaction_type {
                InventoryTransactionType::Normal => Box::from(NormalTransactionData::read(reader)),
                InventoryTransactionType::Mismatch => Box::from(MismatchTransactionData::read(reader)),
                InventoryTransactionType::UseItem => Box::from(UseItemTransactionData::read(reader)),
                InventoryTransactionType::UseItemOnEntity => Box::from(UseItemOnEntityTransactionData::read(reader)),
                InventoryTransactionType::ReleaseItem => Box::from(ReleaseItemTransactionData::read(reader)),
            },
        }
    }
}

/// Sent by the client to the server and the server to the client to make the other side aware of the new item that an
/// entity is holding. It is used to show the item in the hand of entities such as zombies too.
#[derive(Debug)]
pub struct MobEquipment {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The new item held after sending the MobEquipment packet. The entity will be shown holding that item to the
    /// player it was sent to.
    pub new_item: ItemInstance,
    /// The slot in the inventory that was held. This is the same as hotbar_slot, and only remains for backwards
    /// compatibility.
    pub inventory_slot: u8,
    /// The slot in the hot bar that was held. It is the same as InventorySlot, which is only here for backwards
    /// compatibility purposes.
    pub hotbar_slot: u8,
    /// The window ID of the window that had its equipped item changed. This is usually the window ID of the normal
    /// inventory, but may also be something else, for example with the off hand.
    pub window_id: u8,
}

impl Packet for MobEquipment {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        self.new_item.write(writer);

        writer.u8(self.inventory_slot);
        writer.u8(self.hotbar_slot);
        writer.u8(self.window_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),

            new_item: ItemInstance::read(reader),

            inventory_slot: reader.u8(),
            hotbar_slot: reader.u8(),
            window_id: reader.u8(),
        }
    }
}

/// Sent by the server to the client to update the armour an entity is wearing. It is sent for both players and other
/// entities, such as zombies.
#[derive(Debug)]
pub struct MobArmourEquipment {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The equipped helmet of the entity. Items that are not wearable on the head will not be rendered by the client.
    /// Unlike in Java Edition, blocks cannot be worn.
    pub helmet: ItemInstance,
    /// Chestplate is the chestplate of the entity. Items that are not wearable as chestplate will not be rendered.
    pub chestplate: ItemInstance,
    /// Leggings are the leggings of the entity. Items that are not wearable as leggings will not be rendered.
    pub leggings: ItemInstance,
    /// Boots are the boots of the entity. Items that are not wearable as boots will not be rendered.
    pub boots: ItemInstance,
}

impl Packet for MobArmourEquipment {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);

        self.helmet.write(writer);
        self.chestplate.write(writer);
        self.leggings.write(writer);
        self.boots.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),

            helmet: ItemInstance::read(reader),
            chestplate: ItemInstance::read(reader),
            leggings: ItemInstance::read(reader),
            boots: ItemInstance::read(reader),
        }
    }
}

/// Sent by the client when it interacts with another entity in some way. It used to be used for normal entity and block
/// interaction, but this is no longer the case now.
#[derive(Debug)]
pub struct Interact {
    /// The type of action that was executed by the player.
    pub action_type: InteractionAction,
    /// The runtime ID of the entity that the player interacted with. This is empty for the OpenInventory action type.
    pub target_entity_runtime_id: u64,
    /// Associated with the action type above. For the MouseOverEntity action, this is the position relative to the
    /// entity moused over over which the player hovered with its mouse/touch. For the LeaveVehicle, this is the
    /// position that the player spawns at after leaving the vehicle.
    pub position: Vec3,
}

impl Packet for Interact {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());
        writer.var_u64(self.target_entity_runtime_id);
        match self.action_type {
            InteractionAction::MouseOverEntity | InteractionAction::LeaveVehicle => {
                writer.vec3(self.position);
            }
            _ => {}
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            target_entity_runtime_id: reader.var_u64(),
            position: if action_type == InteractionAction::MouseOverEntity || action_type == InteractionAction::LeaveVehicle {
                reader.vec3()
            } else {
                Vec3::default()
            },
        }
    }
}

/// Sent by the client when it requests to pick a block in the world and place its item in their inventory.
#[derive(Debug)]
pub struct BlockPickRequest {
    /// The position at which the client requested to pick the block. The block at that position should have its item
    /// put in HotBarSlot if it is empty.
    pub position: BlockPos,
    /// Specifies if the item should get all NBT tags from the block, meaning the item places a block practically
    /// always equal to the one picked.
    pub add_block_nbt: bool,
    /// The slot that was held at the time of picking a block.
    pub hotbar_slot: u8,
}

impl Packet for BlockPickRequest {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.bool(self.add_block_nbt);
        writer.u8(self.hotbar_slot);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            add_block_nbt: reader.bool(),
            hotbar_slot: reader.u8(),
        }
    }
}

/// Sent by the client when it tries to pick an entity, so that it gets a spawn egg which can spawn that entity.
#[derive(Debug)]
pub struct ActorPickRequest {
    /// The unique ID of the entity that was attempted to be picked. The server must find the type of that entity and
    /// provide the correct spawn egg to the player.
    pub entity_unique_id: i64,
    /// The held hot bar slot of the player at the time of trying to pick the entity. If empty, the resulting spawn egg
    /// should be put into this slot.
    pub hotbar_slot: u8,
    /// True if the pick request requests the entity metadata.
    pub with_data: bool,
}

impl Packet for ActorPickRequest {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(self.hotbar_slot);
        writer.bool(self.with_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            hotbar_slot: reader.u8(),
            with_data: reader.bool(),
        }
    }
}

/// Sent by the client when it executes any action, for example starting to sprint, swim, starting the breaking of a
/// block, dropping an item, etc.
#[derive(Debug)]
pub struct PlayerAction {
    /// The runtime ID of the player. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The type of action that was executed by the player.
    pub action_type: PlayerActionType,
    /// The position of the target block, if the action with the ActionType set concerned a block. If that is not the
    /// case, the block position will be zero.
    pub block_position: BlockPos,
    /// The position of the action's result. When a UseItemOn action is sent, this is the position of the block clicked,
    /// but when a block is placed, this is the position at which the block will be placed.
    pub result_position: BlockPos,
    /// The face of the target block that was touched. If the action with the ActionType set concerned a block. If not,
    /// the face is always zero.
    pub block_face: i32,
}

impl Packet for PlayerAction {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.var_i32(num::ToPrimitive::to_i32(&self.action_type).unwrap());
        writer.u_block_pos(self.block_position);
        writer.u_block_pos(self.result_position);
        writer.var_i32(self.block_face);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            action_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            block_position: reader.u_block_pos(),
            result_position: reader.u_block_pos(),
            block_face: reader.var_i32(),
        }
    }
}

/// Sent by the server to damage the player's armour after being hit. The packet should never be used by servers as it
/// hands the responsibility over to the player completely, while the server can easily reliably update the armour
/// damage of players itself.
#[derive(Debug)]
pub struct HurtArmour {
    /// The cause of the damage dealt to the armour.
    pub cause: i32,
    /// The amount of damage points that was dealt to the player. The damage to the armour will be calculated by the
    /// client based upon this damage, and will also be based upon any enchantments that the armour may have.
    pub damage: i32,
    /// A bitset of all armour slots affected.
    pub armour_slots: i64,
}

impl Packet for HurtArmour {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.cause);
        writer.var_i32(self.damage);
        writer.var_i64(self.armour_slots);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            cause: reader.var_i32(),
            damage: reader.var_i32(),
            armour_slots: reader.var_i64(),
        }
    }
}

/// The server to update the entity metadata of an entity. It includes flags such as if the entity is on fire, but
/// also properties such as the air it has left until it starts drowning.
#[derive(Debug)]
pub struct SetActorData {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A map of entity metadata, which includes flags and data properties that alter in particular the way the player
    /// looks. Flags include ones such as 'on fire' and 'sprinting'. The meta values are indexed by their property key.
    // TODO: Implement entity metadata.
    // pub entity_metadata: dyn Any,
    /// A list of properties that the entity inhibits. These properties define specific attributes of the entity.
    // TODO: Implement entity properties.
    // pub entity_properties: dyn Any,
    /// The server tick at which the packet was sent. It is used in relation to CorrectPlayerMovePrediction.
    pub tick: u64,
}

impl Packet for SetActorData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        // TODO: Implement entity metadata.
        // TODO: Implement entity properties.
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            // entity_metadata: {
            //     // TODO: Implement entity metadata.
            // },
            // entity_properties: {
            //     // TODO: Implement entity properties.
            // },
            tick: reader.var_u64(),
        }
    }
}

/// Sent by the server to change the client-side velocity of an entity. It is usually used in combination with
/// server-side movement calculation.
#[derive(Debug)]
pub struct SetActorMotion {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The new velocity the entity gets. This velocity will initiate the client-side movement of the entity.
    pub velocity: Vec3,
}

impl Packet for SetActorMotion {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.vec3(self.velocity);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            velocity: reader.vec3(),
        }
    }
}

/// Sent by the server to initiate an entity link client-side, meaning one entity will start riding another.
#[derive(Debug)]
pub struct SetActorLink {
    /// The link to be set client-side. It links two entities together, so that one entity rides another. Note that
    /// players that see those entities later will not see the link, unless it is also sent in the AddActor and
    /// AddPlayer packets.
    pub entity_link: EntityLink,
}

impl Packet for SetActorLink {
    fn write(&self, writer: &mut Writer) {
        self.entity_link.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_link: EntityLink::read(reader) }
    }
}

/// Sent by the server. It sets the health of the player it is sent to. The SetHealth packet should no longer be used.
/// Instead, the health attribute should be used so that the health and maximum health may be changed directly.
#[derive(Debug)]
pub struct SetHealth {
    /// The new health of the player.
    pub health: i32,
}

impl Packet for SetHealth {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.health);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { health: reader.var_i32() }
    }
}

/// Sent by the server to update the spawn position of a player, for example when sleeping in a bed.
#[derive(Debug)]
pub struct SetSpawnPosition {
    /// Specifies the behaviour of the spawn set. If World is set, the position that compasses will point to is changed.
    pub spawn_type: SpawnType,
    /// The new position of the spawn that was set. If the spawn type is World, compasses will point to this position.
    /// As of 1.16, position is always the position of the player.
    pub position: BlockPos,
    /// The dimension that had its spawn updated. This is specifically relevant for behaviour added in 1.16 such as the
    /// respawn anchor, which allows setting the spawn in a specific dimension.
    pub dimension: Dimension,
    /// A new field added in 1.16. It holds the spawn position of the world. This spawn position is
    /// {-i32::MIN, -i32::MIN, -i32::MIN} for a default spawn position.
    pub spawn_position: BlockPos,
}

impl Packet for SetSpawnPosition {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.spawn_type).unwrap());
        writer.u_block_pos(self.position);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.u_block_pos(self.spawn_position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            spawn_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.u_block_pos(),
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            spawn_position: reader.u_block_pos(),
        }
    }
}

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
    pub event_data: Box<dyn EventData>,
}

impl Packet for Event {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.var_i32(num::ToPrimitive::to_i32(&self.event_data.event_type()).unwrap());
        writer.u8(self.use_player_id);
        self.event_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        let entity_runtime_id = reader.var_u64();
        let event_type = num::FromPrimitive::from_i32(reader.var_i32()).unwrap();
        Self {
            entity_runtime_id,
            use_player_id: reader.u8(),
            event_data: match event_type {
                EventType::AchievementAwarded => Box::from(AchievementAwardedEventData::read(reader)),
                EventType::EntityInteract => Box::from(EntityInteractEventData::read(reader)),
                EventType::PortalBuilt => Box::from(PortalBuiltEventData::read(reader)),
                EventType::PortalUsed => Box::from(PortalUsedEventData::read(reader)),
                EventType::MobKilled => Box::from(MobKilledEventData::read(reader)),
                EventType::CauldronUsed => Box::from(CauldronUsedEventData::read(reader)),
                EventType::PlayerDied => Box::from(PlayerDiedEventData::read(reader)),
                EventType::BossKilled => Box::from(BossKilledEventData::read(reader)),
                EventType::AgentCommand => Box::from(AgentCommandEventData::read(reader)),
                EventType::AgentCreated => Box::from(()),
                EventType::PatternRemoved => Box::from(PatternRemovedEventData::read(reader)),
                EventType::SlashCommandExecuted => Box::from(SlashCommandExecutedEventData::read(reader)),
                EventType::FishBucketed => Box::from(FishBucketedEventData::read(reader)),
                EventType::MobBorn => Box::from(MobBornEventData::read(reader)),
                EventType::PetDied => Box::from(PetDiedEventData::read(reader)),
                EventType::CauldronInteract => Box::from(CauldronInteractEventData::read(reader)),
                EventType::ComposterInteract => Box::from(ComposterInteractEventData::read(reader)),
                EventType::BellUsed => Box::from(BellUsedEventData::read(reader)),
                EventType::EntityDefinitionTrigger => Box::from(EntityDefinitionTriggerEventData::read(reader)),
                EventType::RaidUpdate => Box::from(RaidUpdateEventData::read(reader)),
                EventType::MovementAnomaly => Box::from(MovementAnomalyEventData::read(reader)),
                EventType::MovementCorrected => Box::from(MovementCorrectedEventData::read(reader)),
                EventType::ExtractHoney => Box::from(ExtractHoneyEventData::read(reader)),
                EventType::TargetBlockHit => Box::from(()), // TODO: TargetBlockHitEventData::read(reader),
                EventType::PiglinBarter => Box::from(()), // TODO: PiglinBarterEventData::read(reader)
                EventType::PlayerWaxedOrUnwaxedCopper => Box::from(PlayerWaxedOrUnwaxedCopperEventData::read(reader)),
                EventType::CodeBuilderRuntimeAction => Box::from(()), // TODO: CodeBuilderRuntimeActionEventData::read(reader)
                EventType::CodeBuilderScoreboard => Box::from(()), // TODO: CodeBuilderScoreboardEventData::read(reader)
                EventType::StriderRiddenInLavaInOverworld => Box::from(()), // TODO: StriderRiddenInLavaInOverworldEventData::read(reader)
                EventType::SneakCloseToSculkSensor => Box::from(SneakCloseToSculkSensorEventData::read(reader)),
            },
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
        for command in self.commands {
            for alias in command.aliases {
                if !indices.contains_key(&alias) {
                    indices.insert(alias.clone(), values.len());
                    values.push(alias);
                }
            }
            for overload in command.overloads {
                for parameter in overload.parameters {
                    for option in parameter.command_enum.options {
                        if !indices.contains_key(&option) {
                            indices.insert(option.clone(), values.len());
                            values.push(option);
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
        for command in self.commands {
            for overload in command.overloads {
                for parameter in overload.parameters {
                    if !parameter.suffix.is_empty() {
                        if !indices.contains_key(&parameter.suffix) {
                            indices.insert(parameter.suffix.clone(), values.len());
                            values.push(parameter.suffix);
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
        for command in self.commands {
            for overload in command.overloads {
                for parameter in overload.parameters {
                    if parameter.command_enum.dynamic {
                        if !indices.contains_key(&parameter.command_enum.enum_type) {
                            indices.insert(parameter.command_enum.enum_type.clone(), values.len());
                            values.push(parameter.command_enum);
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
    pub window_id: u8,
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
        writer.u8(self.window_id);
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
            window_id: reader.u8(),
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
    pub window_id: u8,
    pub window_type: u8,
    pub size: i32,
    pub entity_unique_id: i64,
    pub serialised_inventory_data: Bytes,
}

impl Packet for UpdateEquip {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.window_id);
        writer.u8(self.window_type);
        writer.var_i32(self.size);
        writer.var_i64(self.entity_unique_id);
        writer.bytes(&self.serialised_inventory_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            window_id: reader.u8(),
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

#[derive(Debug)]
pub struct PlaySound {
    pub sound_name: String,
    pub position: Vec3,
    pub volume: f32,
    pub pitch: f32,
}

impl Packet for PlaySound {
    fn write(&self, writer: &mut Writer) {
        let block_pos = BlockPos { x: self.position.x as i32 * 8, y: self.position.y as i32 * 8, z: self.position.z as i32 * 8 };
        writer.string(self.sound_name.as_str());
        writer.block_pos(block_pos);
        writer.f32(self.volume);
        writer.f32(self.pitch);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_name: reader.string(),
            position: {
                let block_pos = reader.block_pos();
                Vec3 { x: block_pos.x as f32 / 8.0, y: block_pos.y as f32 / 8.0, z: block_pos.z as f32 / 8.0 }
            },
            volume: reader.f32(),
            pitch: reader.f32(),
        }
    }
}

#[derive(Debug)]
pub struct StopSound {
    pub sound_name: String,
    pub stop_all: bool,
}

impl Packet for StopSound {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.sound_name.as_str());
        writer.bool(self.stop_all);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_name: reader.string(),
            stop_all: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct SetTitle {
    pub action_type: i32,
    pub text: String,
    pub fade_in_duration: i32,
    pub remain_duration: i32,
    pub fade_out_duration: i32,
    pub xuid: String,
    pub platform_online_id: String,
}

impl Packet for SetTitle {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type);
        writer.string(self.text.as_str());
        writer.var_i32(self.fade_in_duration);
        writer.var_i32(self.remain_duration);
        writer.var_i32(self.fade_out_duration);
        writer.string(self.xuid.as_str());
        writer.string(self.platform_online_id.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_i32(),
            text: reader.string(),
            fade_in_duration: reader.var_i32(),
            remain_duration: reader.var_i32(),
            fade_out_duration: reader.var_i32(),
            xuid: reader.string(),
            platform_online_id: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct AddBehaviourTree {
    pub behaviour_tree: String,
}

impl Packet for AddBehaviourTree {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.behaviour_tree.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            behaviour_tree: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct StructureBlockUpdate {
    pub position: BlockPos,
    pub structure_name: String,
    pub data_field: String,
    pub include_players: bool,
    pub show_bounding_box: bool,
    pub structure_block_type: StructureBlockType,
    pub settings: StructureSettings,
    pub redstone_save_mode: StructureRedstoneSaveMode,
    pub should_trigger: bool,
    pub waterlogged: bool,
}

impl Packet for StructureBlockUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.string(self.structure_name.as_str());
        writer.string(self.data_field.as_str());
        writer.bool(self.include_players);
        writer.bool(self.show_bounding_box);
        writer.var_i32(num::ToPrimitive::to_i32(&self.structure_block_type).unwrap());
        self.settings.write(writer);
        writer.var_i32(num::ToPrimitive::to_i32(&self.redstone_save_mode).unwrap());
        writer.bool(self.should_trigger);
        writer.bool(self.waterlogged);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            structure_name: reader.string(),
            data_field: reader.string(),
            include_players: reader.bool(),
            show_bounding_box: reader.bool(),
            structure_block_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            settings: StructureSettings::read(reader),
            redstone_save_mode: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            should_trigger: reader.bool(),
            waterlogged: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct ShowStoreOffer {
    pub offer_id: String,
    pub show_all: bool,
}

impl Packet for ShowStoreOffer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.offer_id.as_str());
        writer.bool(self.show_all);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            offer_id: reader.string(),
            show_all: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct PurchaseReceipt {
    pub receipts: Vec<String>,
}

impl Packet for PurchaseReceipt {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.receipts.len() as u32);
        self.receipts.iter().for_each(|receipt| writer.string(receipt.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { receipts: (0..reader.var_u32()).map(|_| reader.string()).collect() }
    }
}

#[derive(Debug)]
pub struct PlayerSkin {
    pub uuid: Uuid,
    pub skin: Skin,
    pub new_skin_name: String,
    pub old_skin_name: String,
}

impl Packet for PlayerSkin {
    fn write(&self, writer: &mut Writer) {
        writer.uuid(self.uuid);
        self.skin.write(writer);
        writer.string(self.new_skin_name.as_str());
        writer.string(self.old_skin_name.as_str());
        writer.bool(self.skin.trusted);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            uuid: reader.uuid(),
            skin: Skin::read(reader),
            new_skin_name: reader.string(),
            old_skin_name: reader.string(),
        };
        packet.skin.trusted = reader.bool();

        packet
    }
}

#[derive(Debug)]
pub struct SubClientLogin {
    pub connection_request: Bytes,
}

impl Packet for SubClientLogin {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.connection_request);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            connection_request: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct AutomationClientConnect {
    pub server_uri: String,
}

impl Packet for AutomationClientConnect {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.server_uri.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            server_uri: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct SetLastHurtBy {
    pub entity_type: i32,
}

impl Packet for SetLastHurtBy {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_type: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct BookEdit {
    pub action_type: BookAction,
    pub inventory_slot: u8,
    pub page_number: u8,
    pub secondary_page_number: u8,
    pub text: String,
    pub photo_name: String,
    pub title: String,
    pub author: String,
    pub xuid: String,
}

impl Packet for BookEdit {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());
        writer.u8(self.inventory_slot);
        match self.action_type {
            BookAction::ReplacePage | BookAction::AddPage => {
                writer.u8(self.page_number);
                writer.string(self.text.as_str());
                writer.string(self.photo_name.as_str());
            }
            BookAction::DeletePage => {
                writer.u8(self.page_number);
            }
            BookAction::SwapPages => {
                writer.u8(self.page_number);
                writer.u8(self.secondary_page_number);
            }
            BookAction::Sign => {
                writer.string(self.title.as_str());
                writer.string(self.author.as_str());
                writer.string(self.xuid.as_str());
            }
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            inventory_slot: reader.u8(),
            page_number: if action_type != BookAction::Sign { reader.u8() } else { 0 },
            secondary_page_number: if action_type == BookAction::SwapPages { reader.u8() } else { 0 },
            text: if action_type == BookAction::ReplacePage || action_type == BookAction::AddPage { reader.string() } else { "".to_string() },
            photo_name: if action_type == BookAction::ReplacePage || action_type == BookAction::AddPage { reader.string() } else { "".to_string() },
            title: if action_type == BookAction::Sign { reader.string() } else { "".to_string() },
            author: if action_type == BookAction::Sign { reader.string() } else { "".to_string() },
            xuid: if action_type == BookAction::Sign { reader.string() } else { "".to_string() },
        }
    }
}

#[derive(Debug)]
pub struct NPCRequest {
    pub entity_runtime_id: u64,
    pub request_type: u8,
    pub command_string: String,
    pub action_type: u8,
    pub scene_name: String,
}

impl Packet for NPCRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u8(self.request_type);
        writer.string(self.command_string.as_str());
        writer.u8(self.action_type);
        writer.string(self.scene_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            request_type: reader.u8(),
            command_string: reader.string(),
            action_type: reader.u8(),
            scene_name: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct PhotoTransfer {
    pub photo_name: String,
    pub photo_data: Bytes,
    pub book_id: String,
    pub photo_type: u8,
    pub source_type: u8,
    pub owner_entity_unique_id: i64,
    pub new_photo_name: String,
}

impl Packet for PhotoTransfer {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.photo_name.as_str());
        writer.byte_slice(&self.photo_data);
        writer.string(self.book_id.as_str());
        writer.u8(self.photo_type);
        writer.u8(self.source_type);
        writer.i64(self.owner_entity_unique_id);
        writer.string(self.new_photo_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            photo_name: reader.string(),
            photo_data: reader.byte_slice(),
            book_id: reader.string(),
            photo_type: reader.u8(),
            source_type: reader.u8(),
            owner_entity_unique_id: reader.i64(),
            new_photo_name: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct ModalFormRequest {
    pub form_id: u32,
    pub form_data: Bytes,
}

impl Packet for ModalFormRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.form_id);
        writer.byte_slice(&self.form_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            form_id: reader.var_u32(),
            form_data: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct ModalFormResponse {
    pub form_id: u32,
    pub response_data: Option<Bytes>,
    pub cancel_reason: Option<u8>,
}

impl Packet for ModalFormResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.form_id);
        writer.optional(&self.response_data, |x| writer.byte_slice(&x));
        writer.optional(&self.cancel_reason, writer.u8);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            form_id: reader.var_u32(),
            response_data: reader.optional(reader.byte_slice),
            cancel_reason: reader.optional(reader.u8),
        }
    }
}

#[derive(Debug)]
pub struct ServerSettingsRequest {}

impl Packet for ServerSettingsRequest {
    fn write(&self, _: &mut Writer) {}

    fn read(_: &mut Reader) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ServerSettingsResponse {
    pub form_id: u32,
    pub form_data: Bytes,
}

impl Packet for ServerSettingsResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.form_id);
        writer.byte_slice(&self.form_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            form_id: reader.var_u32(),
            form_data: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct ShowProfile {
    pub xuid: String,
}

impl Packet for ShowProfile {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.xuid.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            xuid: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct SetDefaultGameType {
    pub game_type: i32,
}

impl Packet for SetDefaultGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct RemoveObjective {
    pub objective_name: String,
}

impl Packet for RemoveObjective {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.objective_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            objective_name: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct SetDisplayObjective {
    pub display_slot: String,
    pub objective_name: String,
    pub display_name: String,
    pub criteria_name: String,
    pub sort_order: i32,
}

impl Packet for SetDisplayObjective {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.display_slot.as_str());
        writer.string(self.objective_name.as_str());
        writer.string(self.display_name.as_str());
        writer.string(self.criteria_name.as_str());
        writer.var_i32(self.sort_order);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            display_slot: reader.string(),
            objective_name: reader.string(),
            display_name: reader.string(),
            criteria_name: reader.string(),
            sort_order: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct SetScore {
    pub action_type: ScoreboardAction,
    pub entries: Vec<ScoreboardEntry>,
}

impl Packet for SetScore {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| ScoreboardEntry::read(reader, action_type)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct LabTable {
    pub action_type: u8,
    pub position: BlockPos,
    pub reaction_type: u8,
}

impl Packet for LabTable {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type);
        writer.block_pos(self.position);
        writer.u8(self.reaction_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.u8(),
            position: reader.block_pos(),
            reaction_type: reader.u8(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateBlockSynced {
    pub position: BlockPos,
    pub new_block_runtime_id: u32,
    pub flags: u32,
    pub layer: u32,
    pub entity_unique_id: i64,
    pub transition_type: u64,
}

impl Packet for UpdateBlockSynced {
    fn write(&self, writer: &mut Writer) {
        writer.u_block_pos(self.position);
        writer.var_u32(self.new_block_runtime_id);
        writer.var_u32(self.flags);
        writer.var_u32(self.layer);
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.transition_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.u_block_pos(),
            new_block_runtime_id: reader.var_u32(),
            flags: reader.var_u32(),
            layer: reader.var_u32(),
            entity_unique_id: reader.var_i64(),
            transition_type: reader.var_u64(),
        }
    }
}

#[derive(Debug)]
pub struct MoveActorDelta {
    pub entity_runtime_id: u64,
    pub flags: u16,
    pub position: Vec3,
    pub rotation: Vec3,
}

impl Packet for MoveActorDelta {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        writer.u16(self.flags);
        if self.flags & MoveActorDeltaFlag::HasX.flag() != 0 {
            writer.f32(self.position.x);
        }
        if self.flags & MoveActorDeltaFlag::HasY.flag() != 0 {
            writer.f32(self.position.y);
        }
        if self.flags & MoveActorDeltaFlag::HasZ.flag() != 0 {
            writer.f32(self.position.z);
        }
        if self.flags & MoveActorDeltaFlag::HasRotX.flag() != 0 {
            writer.byte_f32(self.rotation.x);
        }
        if self.flags & MoveActorDeltaFlag::HasRotY.flag() != 0 {
            writer.byte_f32(self.rotation.y);
        }
        if self.flags & MoveActorDeltaFlag::HasRotZ.flag() != 0 {
            writer.byte_f32(self.rotation.z);
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let entity_runtime_id = reader.u64();
        let flags = reader.u16();
        Self {
            entity_runtime_id,
            flags,
            position: {
                let mut position = Vec3::default();
                if flags & MoveActorDeltaFlag::HasX.flag() != 0 {
                    position.x = reader.f32();
                }
                if flags & MoveActorDeltaFlag::HasY.flag() != 0 {
                    position.y = reader.f32();
                }
                if flags & MoveActorDeltaFlag::HasZ.flag() != 0 {
                    position.z = reader.f32();
                }
                position
            },
            rotation: {
                let mut rotation = Vec3::default();
                if flags & MoveActorDeltaFlag::HasRotX.flag() != 0 {
                    rotation.x = reader.byte_f32();
                }
                if flags & MoveActorDeltaFlag::HasRotY.flag() != 0 {
                    rotation.y = reader.byte_f32();
                }
                if flags & MoveActorDeltaFlag::HasRotZ.flag() != 0 {
                    rotation.z = reader.byte_f32();
                }
                rotation
            },
        }
    }
}

#[derive(Debug)]
pub struct SetScoreboardIdentity {
    pub action_type: ScoreboardIdentityAction,
    pub entries: Vec<ScoreboardIdentityEntry>,
}

impl Packet for SetScoreboardIdentity {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action_type).unwrap());

        writer.var_u32(self.entries.len() as u32);
        self.entries.iter().for_each(|entry| entry.write(writer, self.action_type));
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = num::FromPrimitive::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            entries: (0..reader.var_u32()).map(|_| ScoreboardIdentityEntry::read(reader, action_type)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct SetLocalPlayerAsInitialised {
    pub entity_runtime_id: u64,
}

impl Packet for SetLocalPlayerAsInitialised {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateSoftEnum {
    pub enum_type: String,
    pub options: Vec<String>,
    pub action_type: u8,
}

impl Packet for UpdateSoftEnum {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.enum_type.as_str());
        writer.var_u32(self.options.len() as u32);
        self.options.iter().for_each(|option| writer.string(option.as_str()));
        writer.u8(self.action_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            enum_type: reader.string(),
            options: (0..reader.var_u32()).map(|_| reader.string()).collect(),
            action_type: reader.u8(),
        }
    }
}

#[derive(Debug)]
pub struct NetworkStackLatency {
    pub timestamp: i64,
    pub needs_response: bool,
}

impl Packet for NetworkStackLatency {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.timestamp);
        writer.bool(self.needs_response);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            timestamp: reader.i64(),
            needs_response: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct ScriptCustomEvent {
    pub event_name: String,
    pub event_data: u8,
}

impl Packet for ScriptCustomEvent {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.event_name.as_str());
        writer.u8(self.event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_name: reader.string(),
            event_data: reader.u8(),
        }
    }
}

#[derive(Debug)]
pub struct SpawnParticleEffect {
    pub dimension: u8,
    pub entity_unique_id: i64,
    pub position: Vec3,
    pub particle_name: String,
    pub molang_variables: Option<Bytes>,
}

impl Packet for SpawnParticleEffect {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.dimension);
        writer.var_i64(self.entity_unique_id);
        writer.vec3(self.position);
        writer.string(self.particle_name.as_str());
        writer.optional(&self.molang_variables, |x| writer.byte_slice(&x));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: reader.u8(),
            entity_unique_id: reader.var_i64(),
            position: reader.vec3(),
            particle_name: reader.string(),
            molang_variables: reader.optional(reader.byte_slice),
        }
    }
}

#[derive(Debug)]
pub struct AvailableActorIdentifiers {
    pub serialised_entity_identifiers: Bytes,
}

impl Packet for AvailableActorIdentifiers {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.serialised_entity_identifiers);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            serialised_entity_identifiers: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct NetworkChunkPublisherUpdate {
    pub position: BlockPos,
    pub radius: u32,
    pub saved_chunks: Vec<IVec2>,
}

impl Packet for NetworkChunkPublisherUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.var_u32(self.radius);
        writer.u32(self.saved_chunks.len() as u32);
        self.saved_chunks.iter().for_each(|chunk| {
            writer.var_i32(chunk.x);
            writer.var_i32(chunk.y);
        });
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            radius: reader.var_u32(),
            saved_chunks: (0..reader.u32()).map(|_| IVec2::new(reader.var_i32(), reader.var_i32())).collect(),
        }
    }
}

#[derive(Debug)]
pub struct BiomeDefinitionList {
    pub serialised_biome_definitions: Bytes,
}

impl Packet for BiomeDefinitionList {
    fn write(&self, writer: &mut Writer) {
        writer.byte_slice(&self.serialised_biome_definitions);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            serialised_biome_definitions: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct LevelSoundEvent {
    pub sound_type: u32,
    pub position: Vec3,
    pub extra_data: i32,
    pub entity_type: String,
    pub baby_mob: bool,
    pub disable_relative_volume: bool,
}

impl Packet for LevelSoundEvent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.sound_type);
        writer.vec3(self.position);
        writer.var_i32(self.extra_data);
        writer.string(self.entity_type.as_str());
        writer.bool(self.baby_mob);
        writer.bool(self.disable_relative_volume);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            sound_type: reader.var_u32(),
            position: reader.vec3(),
            extra_data: reader.var_i32(),
            entity_type: reader.string(),
            baby_mob: reader.bool(),
            disable_relative_volume: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct LevelEventGeneric {
    pub event_id: i32,
    pub serialised_event_data: Bytes,
}

impl Packet for LevelEventGeneric {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.event_id);
        writer.byte_slice(&self.serialised_event_data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            event_id: reader.var_i32(),
            serialised_event_data: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct LecternUpdate {
    pub page: u8,
    pub page_count: u8,
    pub position: BlockPos,
    pub drop_book: bool,
}

impl Packet for LecternUpdate {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.page);
        writer.u8(self.page_count);
        writer.u_block_pos(self.position);
        writer.bool(self.drop_book);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            page: reader.u8(),
            page_count: reader.u8(),
            position: reader.u_block_pos(),
            drop_book: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct AddEntity {
    pub entity_network_id: u64,
}

impl Packet for AddEntity {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_network_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_network_id: reader.var_u64(),
        }
    }
}

#[derive(Debug)]
pub struct RemoveEntity {
    pub entity_network_id: u64,
}

impl Packet for RemoveEntity {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_network_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_network_id: reader.var_u64(),
        }
    }
}

#[derive(Debug)]
pub struct ClientCacheStatus {
    pub enabled: bool,
}

impl Packet for ClientCacheStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.enabled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            enabled: reader.bool(),
        }
    }
}

/// Sent by the server to create a locked copy of one map into another map. In vanilla, it is used in the cartography
/// table to create a map that is locked and cannot be modified.
#[derive(Debug)]
pub struct MapCreateLockedCopy {
    /// ID of the map that is being copied. The locked copy will obtain all content that is visible on this map, except
    /// the content will not change.
    pub original_map_id: i64,
    /// ID of the map that holds the locked copy of the map that original_map_id points to. Its contents will be
    /// impossible to change.
    pub new_map_id: i64,
}

impl Packet for MapCreateLockedCopy {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.original_map_id);
        writer.var_i64(self.new_map_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            original_map_id: reader.var_i64(),
            new_map_id: reader.var_i64(),
        }
    }
}

/// Sent by the server to show a certain animation on the screen of the player. The packet is used, as an example, for
/// when a raid is triggered and when a raid is defeated.
#[derive(Debug)]
pub struct OnScreenTextureAnimation {
    /// Type of the animation to show. The packet provides no further extra data to allow modifying the duration or
    /// other properties of the animation.
    pub animation_type: i32,
}

impl Packet for OnScreenTextureAnimation {
    fn write(&self, writer: &mut Writer) {
        writer.i32(self.animation_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            animation_type: reader.i32(),
        }
    }
}

/// Sent by the client to request data of a structure.
#[derive(Debug)]
pub struct StructureTemplateDataRequest {
    /// Name of the structure that was set in the structure block's UI. This is the name used to export the structure
    /// to a file.
    pub structure_name: String,
    pub position: BlockPos,
    pub settings: StructureSettings,
    pub request_type: StructureTemplateDataRequestType,
}

impl Packet for StructureTemplateDataRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.structure_name.as_str());
        writer.u_block_pos(self.position);
        self.settings.write(writer);
        writer.u8(num::ToPrimitive::to_u8(&self.request_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            structure_name: reader.string(),
            position: reader.u_block_pos(),
            settings: StructureSettings::read(reader),
            request_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct StructureTemplateDataResponse {
    pub structure_name: String,
    pub success: bool,
    //pub structure_template: dyn Any,
    // TODO: NBT
    pub response_type: StructureTemplateDataResponseType,
}

impl Packet for StructureTemplateDataResponse {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.structure_name.as_str());
        writer.bool(self.success);
        if self.success {
            // TODO: NBT (structure_template)
        }
        writer.u8(num::ToPrimitive::to_u8(&self.response_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        let struct_name = reader.string();
        let success = reader.bool();
        Self {
            structure_name,
            success,
            // TODO: NBT (structure_template) if success
            response_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ClientCacheBlobStatus {
    pub miss_hashes: Vec<u64>,
    pub hit_hashes: Vec<u64>,
}

impl Packet for ClientCacheBlobStatus {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.miss_hashes.len() as u32);
        writer.var_u32(self.hit_hashes.len() as u32);
        self.miss_hashes.iter().for_each(|hash| writer.u64(*hash));
        self.hit_hashes.iter().for_each(|hash| writer.u64(*hash));
    }

    fn read(reader: &mut Reader) -> Self {
        let miss_hashes_len = reader.var_u32();
        let hit_hashes_len = reader.var_u32();
        Self {
            miss_hashes: (0..miss_hashes_len).map(|_| reader.u64()).collect(),
            hit_hashes: (0..hit_hashes_len).map(|_| reader.u64()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct ClientCacheMissResponse {
    pub blobs: Vec<CacheBlob>,
}

impl Packet for ClientCacheMissResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.blobs.len() as u32);
        self.blobs.iter().for_each(|blob| blob.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { blobs: (0..reader.var_u32()).map(|_| CacheBlob::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct EducationSettings {
    pub code_builder_default_uri: String,
    pub code_builder_title: String,
    pub can_resize_code_builder: bool,
    pub disable_legacy_title_bar: bool,
    pub post_process_filter: String,
    pub screenshot_border_path: String,
    pub can_modify_blocks: Option<bool>,
    pub override_uri: Option<String>,
    pub has_quiz: bool,
    pub external_link_settings: Option<EducationExternalLinkSettings>,
}

impl Packet for EducationSettings {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.code_builder_default_uri.as_str());
        writer.string(self.code_builder_title.as_str());
        writer.bool(self.can_resize_code_builder);
        writer.bool(self.disable_legacy_title_bar);
        writer.string(self.post_process_filter.as_str());
        writer.string(self.screenshot_border_path.as_str());
        writer.optional(&self.can_modify_blocks, writer.bool);
        writer.optional(&self.override_uri, writer.string);
        writer.bool(self.has_quiz);
        writer.optional(&self.external_link_settings, |settings| settings.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            code_builder_default_uri: reader.string(),
            code_builder_title: reader.string(),
            can_resize_code_builder: reader.bool(),
            disable_legacy_title_bar: reader.bool(),
            post_process_filter: reader.string(),
            screenshot_border_path: reader.string(),
            can_modify_blocks: reader.optional(reader.bool),
            override_uri: reader.optional(reader.string),
            has_quiz: reader.bool(),
            external_link_settings: reader.optional(|| EducationExternalLinkSettings::read(reader)),
        }
    }
}

#[derive(Debug)]
pub struct Emote {
    pub entity_runtime_id: u64,
    pub emote_id: String,
    pub flags: u8,
}

impl Packet for Emote {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.string(self.emote_id.as_str());
        writer.u8(self.flags);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            emote_id: reader.string(),
            flags: reader.u8(),
        }
    }
}

#[derive(Debug)]
pub struct MultiPlayerSettings {
    pub action_type: i32,
}

impl Packet for MultiPlayerSettings {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct SettingsCommand {
    pub command_line: String,
    pub suppress_output: bool,
}

impl Packet for SettingsCommand {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.command_line.as_str());
        writer.bool(self.suppress_output);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            command_line: reader.string(),
            suppress_output: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct AnvilDamage {
    pub damage: u8,
    pub anvil_position: BlockPos,
}

impl Packet for AnvilDamage {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.damage);
        writer.u_block_pos(self.anvil_position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            damage: reader.u8(),
            anvil_position: reader.u_block_pos(),
        }
    }
}

#[derive(Debug)]
pub struct CompletedUsingItem {
    pub used_item_id: i16,
    pub use_method: i32,
}

impl Packet for CompletedUsingItem {
    fn write(&self, writer: &mut Writer) {
        writer.i16(self.used_item_id);
        writer.i32(self.use_method);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            used_item_id: reader.i16(),
            use_method: reader.i32(),
        }
    }
}

#[derive(Debug)]
pub struct NetworkSettings {
    pub compression_threshold: u16,
    pub compression_algorithm: CompressionType,
    pub client_throttle: bool,
    pub client_throttle_threshold: u8,
    pub client_throttle_scalar: f32,
}

impl Packet for NetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.u16(self.compression_threshold);
        writer.u16(num::ToPrimitive::to_u16(&self.compression_algorithm).unwrap());
        writer.bool(self.client_throttle);
        writer.u8(self.client_throttle_threshold);
        writer.f32(self.client_throttle_scalar);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            compression_threshold: reader.u16(),
            compression_algorithm: num::FromPrimitive::from_u16(reader.u16()).unwrap(),
            client_throttle: reader.bool(),
            client_throttle_threshold: reader.u8(),
            client_throttle_scalar: reader.f32(),
        }
    }
}

#[derive(Debug)]
pub struct PlayerAuthInput {
    pub pitch: f32,
    pub yaw: f32,
    pub position: Vec3,
    pub move_vector: Vec2,
    pub head_yaw: f32,
    pub input_data: u64,
    pub input_mode: u32,
    pub play_mode: PlayMode,
    pub interaction_model: i32,
    pub gaze_direction: Vec3,
    pub tick: u64,
    pub delta: Vec3,
    pub item_interaction_data: UseItemTransactionData,
    pub item_stack_request: ItemStackRequestEntry,
    pub block_actions: Vec<PlayerBlockAction>,
}

impl Packet for PlayerAuthInput {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.vec3(self.position);
        writer.vec2(self.move_vector);
        writer.f32(self.head_yaw);
        writer.var_u64(self.input_data);
        writer.var_u32(self.input_mode);
        writer.var_u32(num::ToPrimitive::to_u32(&self.play_mode).unwrap());
        writer.i32(self.interaction_model);
        if self.play_mode == PlayMode::Reality {
            writer.vec3(self.gaze_direction);
        }
        writer.var_u64(self.tick);
        writer.vec3(self.delta);

        if self.input_data & InputFlag::PerformItemInteraction.flag() != 0 {
            self.item_interaction_data.write_player_action(writer);
        }
        if self.input_data & InputFlag::PerformItemStackRequest.flag() != 0 {
            self.item_stack_request.write(writer);
        }
        if self.input_data & InputFlag::PerformBlockActions.flag() != 0 {
            writer.var_u32(self.block_actions.len() as u32);
            self.block_actions.iter().for_each(|action| action.write(writer));
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            pitch: reader.f32(),
            yaw: reader.f32(),
            position: reader.vec3(),
            move_vector: reader.vec2(),
            head_yaw: reader.f32(),
            input_data: reader.var_u64(),
            input_mode: reader.var_u32(),
            play_mode: num::FromPrimitive::from_u32(reader.var_u32()).unwrap(),
            interaction_model: reader.i32(),
            gaze_direction: Vec3::default(),
            tick: reader.var_u64(),
            delta: reader.vec3(),
            item_interaction_data: Default::default(),
            item_stack_request: Default::default(),
            block_actions: Vec::new(),
        };
        if packet.play_mode == PlayMode::Reality {
            reader.vec3();
        }
        if packet.input_data & InputFlag::PerformItemInteraction.flag() != 0 {
            packet.item_interaction_data = UseItemTransactionData::read_player_action(reader);
        }
        if packet.input_data & InputFlag::PerformItemStackRequest.flag() != 0 {
            packet.item_stack_request = ItemStackRequestEntry::read(reader);
        }
        if packet.input_data & InputFlag::PerformBlockActions.flag() != 0 {
            packet.block_actions = (0..reader.var_u32()).map(|_| PlayerBlockAction::read(reader)).collect();
        }
        packet
    }
}

#[derive(Debug)]
pub struct CreativeContent {
    pub items: Vec<CreativeItem>,
}

impl Packet for CreativeContent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|item| item.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| CreativeItem::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct PlayerEnchantOptions {
    pub options: Vec<EnchantmentOption>,
}

impl Packet for PlayerEnchantOptions {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.options.len() as u32);
        self.options.iter().for_each(|option| option.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { options: (0..reader.var_u32()).map(|_| EnchantmentOption::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct ItemStackRequest {
    pub requests: Vec<ItemStackRequestEntry>,
}

impl Packet for ItemStackRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.requests.len() as u32);
        self.requests.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { requests: (0..reader.var_u32()).map(|_| ItemStackRequestEntry::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct ItemStackResponse {
    pub responses: Vec<ItemStackResponseEntry>,
}

impl Packet for ItemStackResponse {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.responses.len() as u32);
        self.responses.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { responses: (0..reader.var_u32()).map(|_| ItemStackResponseEntry::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct PlayerArmourDamage {
    pub bitset: u8,
    pub helmet_damage: i32,
    pub chestplate_damage: i32,
    pub leggings_damage: i32,
    pub boots_damage: i32,
}

impl Packet for PlayerArmourDamage {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.bitset);
        if self.bitset & 0x01 != 0 {
            writer.i32(self.helmet_damage);
        }
        if self.bitset & 0x02 != 0 {
            writer.i32(self.chestplate_damage);
        }
        if self.bitset & 0x04 != 0 {
            writer.i32(self.leggings_damage);
        }
        if self.bitset & 0x08 != 0 {
            writer.i32(self.boots_damage);
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let bitset = reader.u8();
        Self {
            bitset,
            helmet_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
            chestplate_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
            leggings_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
            boots_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
        }
    }
}

#[derive(Debug)]
pub struct CodeBuilder {
    pub url: String,
    pub should_open_code_builder: bool,
}

impl Packet for CodeBuilder {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.url.as_str());
        writer.bool(self.should_open_code_builder);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            url: reader.string(),
            should_open_code_builder: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct UpdatePlayerGameType {
    pub game_type: i32,
    pub player_unique_id: i64,
}

impl Packet for UpdatePlayerGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.game_type);
        writer.var_i64(self.player_unique_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: reader.var_i32(),
            player_unique_id: reader.var_i64(),
        }
    }
}

#[derive(Debug)]
pub struct EmoteList {
    pub player_runtime_id: u64,
    pub emote_pieces: Vec<Uuid>,
}

impl Packet for EmoteList {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.player_runtime_id);
        writer.var_u32(self.emote_pieces.len() as u32);
        self.emote_pieces.iter().for_each(|emote| writer.uuid(*emote));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_runtime_id: reader.var_u64(),
            emote_pieces: (0..reader.var_u32()).map(|_| reader.uuid()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct PositionTrackingDBServerBroadcast {
    pub broadcast_action: PositionTrackingDBBroadcastAction,
    pub tracking_id: i32,
    //pub payload: dyn Any, // TODO: NBT
}

impl Packet for PositionTrackingDBServerBroadcast {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.broadcast_action).unwrap());
        writer.var_i32(self.tracking_id);
        // TODO: NBT (payload)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            broadcast_action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            tracking_id: reader.var_i32(),
            // payload: {
            //     // TODO: NBT
            // },
        }
    }
}

#[derive(Debug)]
pub struct PositionTrackingDBClientRequest {
    pub request_action: PositionTrackingDBRequestAction,
    pub tracking_id: i32,
}

impl Packet for PositionTrackingDBClientRequest {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.request_action).unwrap());
        writer.var_i32(self.tracking_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            request_action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            tracking_id: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct DebugInfo {
    pub player_unique_id: i64,
    pub data: Bytes,
}

impl Packet for DebugInfo {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.player_unique_id);
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            player_unique_id: reader.var_i64(),
            data: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct PacketViolationWarning {
    pub violation_type: PacketViolationType,
    pub severity: PacketViolationSeverity,
    pub packet_id: i32,
    pub violation_context: String,
}

impl Packet for PacketViolationWarning {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.violation_type).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.severity).unwrap());
        writer.var_i32(self.packet_id);
        writer.string(self.violation_context.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            violation_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            severity: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            packet_id: reader.var_i32(),
            violation_context: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct MotionPredictionHints {
    pub entity_runtime_id: u64,
    pub velocity: Vec3,
    pub on_ground: bool,
}

impl Packet for MotionPredictionHints {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.vec3(self.velocity);
        writer.bool(self.on_ground);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            velocity: reader.vec3(),
            on_ground: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct AnimateEntity {
    pub animation: String,
    pub next_state: String,
    pub stop_condition: String,
    pub stop_condition_version: i32,
    pub controller: String,
    pub blend_out_time: f32,
    pub entity_runtime_ids: Vec<u64>,
}

impl Packet for AnimateEntity {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.animation.as_str());
        writer.string(self.next_state.as_str());
        writer.string(self.stop_condition.as_str());
        writer.i32(self.stop_condition_version);
        writer.string(self.controller.as_str());
        writer.f32(self.blend_out_time);
        writer.var_u32(self.entity_runtime_ids.len() as u32);
        self.entity_runtime_ids.iter().for_each(|runtime_id| writer.var_u64(*runtime_id));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            animation: reader.string(),
            next_state: reader.string(),
            stop_condition: reader.string(),
            stop_condition_version: reader.i32(),
            controller: reader.string(),
            blend_out_time: reader.f32(),
            entity_runtime_ids: (0..reader.var_u32()).map(|_| reader.var_u64()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub shake_type: CameraShakeType,
    pub action: CameraShakeAction,
}

impl Packet for CameraShake {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.intensity);
        writer.f32(self.duration);
        writer.u8(num::ToPrimitive::to_u8(&self.shake_type).unwrap());
        writer.u8(num::ToPrimitive::to_u8(&self.action).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            intensity: reader.f32(),
            duration: reader.f32(),
            shake_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct PlayerFog {
    pub stack: Vec<String>,
}

impl Packet for PlayerFog {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.stack.len() as u32);
        self.stack.iter().for_each(|stack| writer.string(stack.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { stack: (0..reader.var_u32()).map(|_| reader.string()).collect() }
    }
}

#[derive(Debug)]
pub struct CorrectPlayerMovePrediction {
    pub position: Vec3,
    pub delta: Vec3,
    pub on_ground: bool,
    pub tick: u64,
}

impl Packet for CorrectPlayerMovePrediction {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.vec3(self.delta);
        writer.bool(self.on_ground);
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            delta: reader.vec3(),
            on_ground: reader.bool(),
            tick: reader.var_u64(),
        }
    }
}

#[derive(Debug)]
pub struct ItemComponent {
    pub items: Vec<ItemComponentEntry>,
}

impl Packet for ItemComponent {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.items.len() as u32);
        self.items.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { items: (0..reader.var_u32()).map(|_| ItemComponentEntry::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct FilterText {
    pub text: String,
    pub from_server: bool,
}

impl Packet for FilterText {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.text.as_str());
        writer.bool(self.from_server);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            text: reader.string(),
            from_server: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct ClientBoundDebugRenderer {
    pub render_type: ClientBoundDebugRendererType,
    pub text: String,
    pub position: Vec3,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub duration: i64,
}

impl Packet for ClientBoundDebugRenderer {
    fn write(&self, writer: &mut Writer) {
        writer.i32(num::ToPrimitive::to_i32(&self.render_type).unwrap());
        if self.render_type == ClientBoundDebugRendererType::AddCube {
            writer.string(self.text.as_str());
            writer.vec3(self.position);
            writer.f32(self.red);
            writer.f32(self.green);
            writer.f32(self.blue);
            writer.f32(self.alpha);
            writer.i64(self.duration);
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let render_type = num::FromPrimitive::from_i32(reader.i32()).unwrap();
        Self {
            render_type,
            text: if render_type == ClientBoundDebugRendererType::AddCube { reader.string() } else { "".to_string() },
            position: if render_type == ClientBoundDebugRendererType::AddCube { reader.vec3() } else { Vec3::new(0.0, 0.0, 0.0) },
            red: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            green: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            blue: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            alpha: if render_type == ClientBoundDebugRendererType::AddCube { reader.f32() } else { 0.0 },
            duration: if render_type == ClientBoundDebugRendererType::AddCube { reader.i64() } else { 0 },
        }
    }
}

#[derive(Debug)]
pub struct SyncActorProperty {
    //pub property_data: dyn Any, // TODO: NBT
}

impl Packet for SyncActorProperty {
    fn write(&self, writer: &mut Writer) {
        // TODO: NBT (property_data)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            // property_data: {
            //     // TODO: NBT
            // }
        }
    }
}

#[derive(Debug)]
pub struct AddVolumeEntity {
    pub entity_runtime_id: u64,
    //pub entity_metadata: dyn Any,
    // TODO: NBT
    pub encoding_identifier: String,
    pub instance_identifier: String,
    pub bounds: [BlockPos; 2],
    pub dimension: Dimension,
    pub engine_version: String,
}

impl Packet for AddVolumeEntity {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        // TODO: NBT (entity_metadata)(
        writer.string(self.encoding_identifier.as_str());
        writer.string(self.instance_identifier.as_str());
        writer.u_block_pos(self.bounds[0]);
        writer.u_block_pos(self.bounds[1]);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.string(self.engine_version.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.u64(),
            // entity_metadata: {
            //     // TODO: NBT
            // },
            encoding_identifier: reader.string(),
            instance_identifier: reader.string(),
            bounds: [reader.u_block_pos(), reader.u_block_pos()],
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            engine_version: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct RemoveVolumeEntity {
    pub entity_runtime_id: u64,
    pub dimension: Dimension,
}

impl Packet for RemoveVolumeEntity {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_runtime_id);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.u64(),
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct SimulationType {
    pub simulation_type: Simulation,
}

impl Packet for SimulationType {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.simulation_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            simulation_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct NPCDialogue {
    pub entity_unique_id: u64,
    pub action_type: NPCDialogueAction,
    pub dialogue: String,
    pub scene_name: String,
    pub npc_name: String,
    pub action_json: String,
}

impl Packet for NPCDialogue {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_unique_id);
        writer.var_i32(num::ToPrimitive::to_i32(&self.action_type).unwrap());
        writer.string(self.dialogue.as_str());
        writer.string(self.scene_name.as_str());
        writer.string(self.npc_name.as_str());
        writer.string(self.action_json.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.u64(),
            action_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            dialogue: reader.string(),
            scene_name: reader.string(),
            npc_name: reader.string(),
            action_json: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct EducationResourceURI {
    pub resource: EducationSharedResourceURI,
}

impl Packet for EducationResourceURI {
    fn write(&self, writer: &mut Writer) {
        self.resource.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            resource: EducationSharedResourceURI::read(reader),
        }
    }
}

#[derive(Debug)]
pub struct CreatePhoto {
    pub entity_unique_id: i64,
    pub photo_name: String,
    pub item_name: String,
}

impl Packet for CreatePhoto {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.string(self.photo_name.as_str());
        writer.string(self.item_name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            photo_name: reader.string(),
            item_name: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateSubChunkBlocks {
    pub position: BlockPos,
    pub blocks: Vec<BlockChangeEntry>,
    pub extra: Vec<BlockChangeEntry>,
}

impl Packet for UpdateSubChunkBlocks {
    fn write(&self, writer: &mut Writer) {
        writer.block_pos(self.position);
        writer.var_u32(self.blocks.len() as u32);
        self.blocks.iter().for_each(|entry| entry.write(writer));
        writer.var_u32(self.extra.len() as u32);
        self.extra.iter().for_each(|entry| entry.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.block_pos(),
            blocks: (0..reader.var_u32()).map(|_| BlockChangeEntry::read(reader)).collect(),
            extra: (0..reader.var_u32()).map(|_| BlockChangeEntry::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct PhotoInfoRequest {
    pub photo_id: i64,
}

impl Packet for PhotoInfoRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.photo_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            photo_id: reader.var_i64(),
        }
    }
}

#[derive(Debug)]
pub struct SubChunk {
    pub cache_enabled: bool,
    pub dimension: Dimension,
    pub position: BlockPos,
    pub sub_chunk_entries: Vec<SubChunkEntry>,
}

impl Packet for SubChunk {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.cache_enabled);
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.block_pos(self.position);
        writer.u32(self.sub_chunk_entries.len() as u32);
        self.sub_chunk_entries.iter().for_each(|entry| entry.write(writer, self.cache_enabled));
    }

    fn read(reader: &mut Reader) -> Self {
        let cache_enabled = reader.bool();
        Self {
            cache_enabled,
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            sub_chunk_entries: (0..reader.u32()).map(|_| SubChunkEntry::read(reader, pk.cache_enabled)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct SubChunkRequest {
    pub dimension: Dimension,
    pub position: BlockPos,
    pub offsets: Vec<SubChunkOffset>,
}

impl Packet for SubChunkRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.dimension).unwrap());
        writer.block_pos(self.position);

        writer.u32(self.offsets.len() as u32);
        self.offsets.iter().for_each(|offset| offset.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            dimension: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            position: reader.block_pos(),
            offsets: (0..reader.u32()).map(|_| SubChunkOffset::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct ClientStartItemCooldown {
    pub category: String,
    pub duration: i32,
}

impl Packet for ClientStartItemCooldown {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.category.as_str());
        writer.var_i32(self.duration);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            category: reader.string(),
            duration: reader.var_i32(),
        }
    }
}

#[derive(Debug)]
pub struct ScriptMessage {
    pub identifier: String,
    pub data: Bytes,
}

impl Packet for ScriptMessage {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.identifier.as_str());
        writer.byte_slice(&self.data);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            identifier: reader.string(),
            data: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct CodeBuilderSource {
    pub operation: CodeBuilderOperation,
    pub category: CodeBuilderCategory,
    pub value: u8,
}

impl Packet for CodeBuilderSource {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.operation).unwrap());
        writer.u8(num::ToPrimitive::to_u8(&self.category).unwrap());
        writer.u8(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            operation: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            category: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            value: reader.u8(),
        }
    }
}

#[derive(Debug)]
pub struct TickingAreasLoadStatus {
    pub preload: bool,
}

impl Packet for TickingAreasLoadStatus {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.preload);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            preload: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct DimensionData {
    pub definitions: Vec<DimensionDefinition>,
}

impl Packet for DimensionData {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.definitions.len() as u32);
        self.definitions.iter().for_each(|d| d.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { definitions: (0..reader.var_u32()).map(|_| DimensionDefinition::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct AgentAction {
    pub identifier: String,
    pub action: AgentActionType,
    pub response: Bytes,
}

impl Packet for AgentAction {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.identifier.as_str());
        writer.var_i32(num::ToPrimitive::to_i32(&self.action).unwrap());
        writer.byte_slice(&self.response);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            identifier: reader.string(),
            action: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            response: reader.byte_slice(),
        }
    }
}

#[derive(Debug)]
pub struct ChangeMobProperty {
    pub entity_unique_id: u64,
    pub property: String,
    pub bool_value: bool,
    pub string_value: String,
    pub int_value: i32,
    pub float_value: f32,
}

impl Packet for ChangeMobProperty {
    fn write(&self, writer: &mut Writer) {
        writer.u64(self.entity_unique_id);
        writer.string(self.property.as_str());
        writer.bool(self.bool_value);
        writer.string(self.string_value.as_str());
        writer.var_i32(self.int_value);
        writer.f32(self.float_value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.u64(),
            property: reader.string(),
            bool_value: reader.bool(),
            string_value: reader.string(),
            int_value: reader.var_i32(),
            float_value: reader.f32(),
        }
    }
}

#[derive(Debug)]
pub struct LessonProgress {
    pub action: LessonAction,
    pub score: i32,
    pub identifier: String,
}

impl Packet for LessonProgress {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.action).unwrap());
        writer.var_i32(self.score);
        writer.string(self.identifier.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            score: reader.var_i32(),
            identifier: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct RequestAbility {
    pub ability: Ability,
    //pub value: dyn Any,
}

impl Packet for RequestAbility {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.ability).unwrap());
        //writer.write_TODO(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            ability: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            //value: reader.read_TODO(),
        }
    }
}

#[derive(Debug)]
pub struct RequestPermissions {
    pub entity_unique_id: i64,
    pub permission_level: PermissionLevel,
    pub requested_permissions: u16,
}

impl Packet for RequestPermissions {
    fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(num::ToPrimitive::to_u8(&self.permission_level).unwrap());
        writer.u16(self.requested_permissions);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            permission_level: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            requested_permissions: reader.u16(),
        }
    }
}

#[derive(Debug)]
pub struct ToastRequest {
    pub title: String,
    pub message: String,
}

impl Packet for ToastRequest {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.title.as_str());
        writer.string(self.message.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            title: reader.string(),
            message: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateAbilities {
    pub ability_data: AbilityData,
}

impl Packet for UpdateAbilities {
    fn write(&self, writer: &mut Writer) {
        self.ability_data.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            ability_data: AbilityData::read(reader),
        }
    }
}

#[derive(Debug)]
pub struct UpdateAdventureSettings {
    pub no_pvm: bool,
    pub no_mvp: bool,
    pub immutable_world: bool,
    pub show_name_tags: bool,
    pub auto_jump: bool,
}

impl Packet for UpdateAdventureSettings {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.no_pvm);
        writer.bool(self.no_mvp);
        writer.bool(self.immutable_world);
        writer.bool(self.show_name_tags);
        writer.bool(self.auto_jump);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            no_pvm: reader.bool(),
            no_mvp: reader.bool(),
            immutable_world: reader.bool(),
            show_name_tags: reader.bool(),
            auto_jump: reader.bool(),
        }
    }
}

#[derive(Debug)]
pub struct DeathInfo {
    pub cause: String,
    pub messages: Vec<String>,
}

impl Packet for DeathInfo {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.cause.as_str());
        writer.var_u32(self.messages.len() as u32);
        self.messages.iter().for_each(|m| writer.string(m.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            cause: reader.string(),
            messages: (0..reader.var_u32()).map(|_| reader.string()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct EditorNetwork {
    //pub payload: dyn Any, // TODO: NBT
}

impl Packet for EditorNetwork {
    fn write(&self, writer: &mut Writer) {
        // TODO: NBT (payload)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            // payload: {
            //     // TODO: NBT
            // }
        }
    }
}

#[derive(Debug)]
pub struct FeatureRegistry {
    pub features: Vec<GenerationFeature>,
}

impl Packet for FeatureRegistry {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.features.len() as u32);
        self.features.iter().for_each(|f| f.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { features: (0..reader.var_u32()).map(|_| GenerationFeature::read(reader)).collect() }
    }
}

#[derive(Debug)]
pub struct ServerStats {
    pub server_time: f32,
    pub network_time: f32,
}

impl Packet for ServerStats {
    fn write(&self, writer: &mut Writer) {
        writer.f32(self.server_time);
        writer.f32(self.network_time);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            server_time: reader.f32(),
            network_time: reader.f32(),
        }
    }
}

#[derive(Debug)]
pub struct RequestNetworkSettings {
    pub client_protocol: i32,
}

impl Packet for RequestNetworkSettings {
    fn write(&self, writer: &mut Writer) {
        writer.i32_be(self.client_protocol);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            client_protocol: reader.i32_be(),
        }
    }
}

#[derive(Debug)]
pub struct GameTestRequest {
    pub max_tests_per_batch: i32,
    pub repetitions: i32,
    pub rotation: GameTestRequestRotation,
    pub stop_on_error: bool,
    pub position: BlockPos,
    pub tests_per_row: i32,
    pub name: String,
}

impl Packet for GameTestRequest {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.max_tests_per_batch);
        writer.var_i32(self.repetitions);
        writer.u8(num::ToPrimitive::to_u8(&self.rotation).unwrap());
        writer.bool(self.stop_on_error);
        writer.block_pos(self.position);
        writer.var_i32(self.tests_per_row);
        writer.string(self.name.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            max_tests_per_batch: reader.var_i32(),
            repetitions: reader.var_i32(),
            rotation: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            stop_on_error: reader.bool(),
            position: reader.block_pos(),
            tests_per_row: reader.var_i32(),
            name: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct GameTestResults {
    pub name: String,
    pub succeeded: bool,
    pub error: String,
}

impl Packet for GameTestResults {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.name.as_str());
        writer.bool(self.succeeded);
        writer.string(self.error.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            name: reader.string(),
            succeeded: reader.bool(),
            error: reader.string(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateClientInputLocks {
    pub locks: u32,
    pub position: Vec3,
}

impl Packet for UpdateClientInputLocks {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.locks);
        writer.vec3(self.position);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            locks: reader.var_u32(),
            position: reader.vec3(),
        }
    }
}
