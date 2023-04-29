use crate::chan::PkReceiver;
use crate::client::auth;
use crate::compression::Compression;
use crate::connection::{ConnError, Connection, ExpectedPackets, Sequence};
use crate::proto;
use crate::proto::io::NBT;
use crate::proto::packet::biome_definition_list::BiomeDefinitionList;
use crate::proto::packet::chunk_radius_updated::ChunkRadiusUpdated;
use crate::proto::packet::client_cache_status::ClientCacheStatus;
use crate::proto::packet::creative_content::CreativeContent;
use crate::proto::packet::disconnect::Disconnect;
use crate::proto::packet::login::Login;
use crate::proto::packet::network_settings::NetworkSettings;
use crate::proto::packet::play_status::{PlayStatus, PlayStatusType};
use crate::proto::packet::request_chunk_radius::RequestChunkRadius;
use crate::proto::packet::request_network_settings::RequestNetworkSettings;
use crate::proto::packet::resource_pack_client_response::ResourcePackClientResponse;
use crate::proto::packet::resource_pack_stack::ResourcePackStack;
use crate::proto::packet::resource_packs_info::ResourcePacksInfo;
use crate::proto::packet::set_local_player_as_initialised::SetLocalPlayerAsInitialised;
use crate::proto::packet::start_game::{
    ChatRestrictionLevel, EducationEditionRegion, GamePublishSetting, SpawnBiomeType, StartGame,
};
use crate::proto::packet::Packet;
use crate::proto::types::education::EducationSharedResourceURI;
use crate::proto::types::game_rule::{GameRule, GameRuleValue};
use crate::proto::types::player::PlayerMovementSettings;
use crate::proto::types::resource_pack::ResourcePackResponse;
use crate::proto::types::world::{Difficulty, Dimension, GameType, Generator, PermissionLevel};
use async_trait::async_trait;
use base64ct::Encoding;
use bytes::Bytes;
use rand::random;
use uuid::Uuid;
use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_nbt::Value;

/// The server to client login sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct LoginSequence {
    /// Whether to enforce xbox authentication for the player joining.
    ///
    /// If set to true, the validity of the user's xbox auth will be checked. If set to false,
    /// anyone will be able to join the server with any username.
    pub xbox_auth: bool,
    /// The settings to use for packet compression.
    pub compression: CompressionSettings,
}

/// Settings for packet compression.
#[derive(Debug, Clone, PartialEq)]
pub struct CompressionSettings {
    /// The minimum size a packet batch needs to be in order for it to get compressed by the client.
    ///
    /// Setting this to zero will cause no packets to be compressed. They will however still be
    /// valid compressed data for the compression algorithm used.
    pub threshold: u16,
    /// The compression algorithm to use to compress packet batches.
    pub algorithm: Compression,
}

impl Default for LoginSequence {
    fn default() -> Self {
        Self {
            xbox_auth: true,
            compression: CompressionSettings::default(),
        }
    }
}

impl Default for CompressionSettings {
    fn default() -> Self {
        Self {
            threshold: 512,
            algorithm: Compression::Deflate,
        }
    }
}

#[async_trait]
impl Sequence<Result<(), ConnError>> for LoginSequence {
    async fn execute<'b>(
        self,
        mut reader: PkReceiver,
        conn: &'b Connection,
        expectancies: &'b ExpectedPackets,
    ) -> Result<(), ConnError> {
        // Phase 1: Network settings.
        {
            expectancies.queue::<RequestNetworkSettings>().await;
            let req_net_set = RequestNetworkSettings::try_from(reader.recv().await).unwrap();
            // Disconnect the player if the protocol does not match.
            if req_net_set.client_protocol.0 != proto::CURRENT_PROTOCOL {
                conn.write_packet(&Packet::from(Disconnect {
                    message: Some(format!(
                        "Incompatible client version: expected {}, got {}.",
                        proto::CURRENT_PROTOCOL,
                        req_net_set.client_protocol.0
                    )),
                }))
                .await;
                conn.flush().await?;
                conn.close().await?;
                return Ok(()); // todo: return something other than this
            }

            expectancies.queue::<Login>().await;

            conn.write_packet(&Packet::from(NetworkSettings {
                compression_threshold: self.compression.threshold.clone(),
                compression_algorithm: self.compression.algorithm.clone(),
                client_throttle: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            }))
            .await;
            conn.flush().await?;
            conn.set_compression(self.compression.algorithm.clone())
                .await;
        }

        // Phase 2: Login.
        {
            let login = Login::try_from(reader.recv().await).unwrap();
            if self.xbox_auth {
                // todo: verify login request
            }
        }

        // Phase 3: Encryption.
        {
            // todo
        }

        expectancies.queue::<ClientCacheStatus>().await;

        conn.write_packet(&Packet::from(PlayStatus {
            status: PlayStatusType::LoginSuccess,
        }))
        .await;
        conn.flush().await?;

        let cache_status = ClientCacheStatus::try_from(reader.recv().await).unwrap();
        println!("Client cache: {}", cache_status.enabled);

        expectancies.queue::<ResourcePackClientResponse>().await;

        conn.write_packet(&Packet::from(ResourcePacksInfo {
            texture_pack_required: false,
            has_scripts: false,
            forcing_server_packs: false,
            behaviour_packs: vec![],
            texture_packs: vec![],
        }))
        .await;
        conn.flush().await?;

        let resp = ResourcePackClientResponse::try_from(reader.recv().await).unwrap();
        if resp.response != ResourcePackResponse::AllPacksDownloaded {
            todo!()
        }

        expectancies.queue::<ResourcePackClientResponse>().await;
        conn.write_packet(
            &ResourcePackStack {
                texture_pack_required: false,
                behaviour_packs: vec![],
                texture_packs: vec![],
                base_game_version: proto::CURRENT_VERSION.to_string(),
                experiments: vec![],
                experiments_previously_toggled: false,
            }
            .into(),
        )
        .await;

        let resp = ResourcePackClientResponse::try_from(reader.recv().await).unwrap();
        if resp.response != ResourcePackResponse::Completed {
            todo!()
        }

        expectancies.queue::<RequestChunkRadius>().await;

        let rid = 1i64;
        conn.write_packet(&Packet::from(StartGame {
            entity_unique_id: (1i64).into(),
            entity_runtime_id: (1u64).into(),
            player_game_mode: GameType::Survival,
            player_position: Default::default(),
            pitch: 0.0,
            yaw: 0.0,
            world_seed: 351235,
            spawn_biome_type: SpawnBiomeType::Default,
            user_defined_biome_name: "".to_string(),
            dimension: Dimension::Overworld,
            generator: Generator::Overworld,
            world_game_mode: GameType::Survival,
            difficulty: Difficulty::Peaceful,
            world_spawn: Default::default(),
            achievements_disabled: true,
            editor_world: false,
            created_in_editor: false,
            exported_from_editor: false,
            day_cycle_lock_time: Default::default(),
            education_edition_offer: EducationEditionRegion::None,
            education_features_enabled: true,
            education_product_id: "".to_string(),
            rain_level: 0.0,
            lightning_level: 0.0,
            confirmed_platform_locked_content: false,
            multi_player_game: true,
            lan_broadcast_enabled: true,
            xbl_broadcast_mode: GamePublishSetting::None,
            platform_broadcast_mode: GamePublishSetting::None,
            commands_enabled: true,
            texture_pack_required: false,
            game_rules: vec![GameRule {
                name: "naturalregeneration".to_string(),
                can_be_modified_by_player: false,
                value: GameRuleValue::Bool(false),
            }],
            experiments: vec![],
            experiments_previously_toggled: false,
            bonus_chest_enabled: false,
            start_with_map_enabled: false,
            player_permissions: PermissionLevel::Member,
            server_chunk_tick_radius: 0,
            has_locked_behaviour_pack: false,
            has_locked_texture_pack: false,
            from_locked_world_template: false,
            msa_gamer_tags_only: false,
            from_world_template: false,
            world_template_settings_locked: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: true,
            base_game_version: proto::CURRENT_VERSION.to_string(),
            limited_world_width: 0,
            limited_world_depth: 0,
            new_nether: false,
            education_shared_resource_uri: EducationSharedResourceURI {
                button_name: "".to_string(),
                link_uri: "".to_string(),
            },
            force_experimental_gameplay: None,
            chat_restriction_level: ChatRestrictionLevel::None,
            disable_player_interactions: false,
            level_id: "".to_string(),
            world_name: "Zuri Server".to_string(),
            template_content_identity: "".to_string(),
            trial: false,
            player_movement_settings: PlayerMovementSettings {
                movement_type: Default::default(),
                rewind_history_size: Default::default(),
                server_authoritative_block_breaking: true,
            },
            time: 0,
            enchantment_seed: Default::default(),
            blocks: vec![],
            items: vec![],
            multi_player_correlation_id: Uuid::from_u128(random()).to_string(),
            server_authoritative_inventory: true,
            game_version: proto::CURRENT_VERSION.to_string(),
            property_data: NBT::from(Value::Compound(Default::default())),
            server_block_state_checksum: 0,
            world_template_id: Default::default(),
            client_side_generation: false,
            use_block_network_id_hashes: false,
        }))
        .await;
        conn.flush().await?;

        let req_chunk_radius = RequestChunkRadius::try_from(reader.recv().await).unwrap();

        conn.write_packet(&Packet::from(ChunkRadiusUpdated {
            chunk_radius: req_chunk_radius.chunk_radius.0.min(10).into(),
        }))
        .await;

        // If the client does not receive this packet, it will crash.
        let biomes = "CgAKDWJhbWJvb19qdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoTYmFtYm9vX2p1bmdsZV9oaWxscwUIZG93bmZhbGxmZmY/BQt0ZW1wZXJhdHVyZTMzcz8ACgViZWFjaAUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACgxiaXJjaF9mb3Jlc3QFCGRvd25mYWxsmpkZPwULdGVtcGVyYXR1cmWamRk/AAoSYmlyY2hfZm9yZXN0X2hpbGxzBQhkb3duZmFsbJqZGT8FC3RlbXBlcmF0dXJlmpkZPwAKGmJpcmNoX2ZvcmVzdF9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFGJpcmNoX2ZvcmVzdF9tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKCmNvbGRfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw9AAoKY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgpjb2xkX3RhaWdhBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKEGNvbGRfdGFpZ2FfaGlsbHMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmUAAAC/AAoSY29sZF90YWlnYV9tdXRhdGVkBQhkb3duZmFsbM3MzD4FC3RlbXBlcmF0dXJlAAAAvwAKD2RlZXBfY29sZF9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8AChFkZWVwX2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChNkZWVwX2x1a2V3YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCmRlZXBfb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAoPZGVlcF93YXJtX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKBmRlc2VydAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgxkZXNlcnRfaGlsbHMFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoOZGVzZXJ0X211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoNZXh0cmVtZV9oaWxscwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4AChJleHRyZW1lX2hpbGxzX2VkZ2UFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoVZXh0cmVtZV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbJqZmT4FC3RlbXBlcmF0dXJlzcxMPgAKGGV4dHJlbWVfaGlsbHNfcGx1c190cmVlcwUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACiBleHRyZW1lX2hpbGxzX3BsdXNfdHJlZXNfbXV0YXRlZAUIZG93bmZhbGyamZk+BQt0ZW1wZXJhdHVyZc3MTD4ACg1mbG93ZXJfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKBmZvcmVzdAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgxmb3Jlc3RfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzMzM/AAoMZnJvemVuX29jZWFuBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAAAAKDGZyb3plbl9yaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACgRoZWxsBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDWljZV9tb3VudGFpbnMFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAAAAAoKaWNlX3BsYWlucwUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAAChFpY2VfcGxhaW5zX3NwaWtlcwUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZQAAAAAACgZqdW5nbGUFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoLanVuZ2xlX2VkZ2UFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUzM3M/AAoTanVuZ2xlX2VkZ2VfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzcz8ACgxqdW5nbGVfaGlsbHMFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoOanVuZ2xlX211dGF0ZWQFCGRvd25mYWxsZmZmPwULdGVtcGVyYXR1cmUzM3M/AAoTbGVnYWN5X2Zyb3plbl9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAAAACg5sdWtld2FybV9vY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgptZWdhX3RhaWdhBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKEG1lZ2FfdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmWamZk+AAoEbWVzYQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAACgptZXNhX2JyeWNlBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlAAAAQAAKDG1lc2FfcGxhdGVhdQUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChRtZXNhX3BsYXRlYXVfbXV0YXRlZAUIZG93bmZhbGwAAAAABQt0ZW1wZXJhdHVyZQAAAEAAChJtZXNhX3BsYXRlYXVfc3RvbmUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoabWVzYV9wbGF0ZWF1X3N0b25lX211dGF0ZWQFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAABAAAoPbXVzaHJvb21faXNsYW5kBQhkb3duZmFsbAAAgD8FC3RlbXBlcmF0dXJlZmZmPwAKFW11c2hyb29tX2lzbGFuZF9zaG9yZQUIZG93bmZhbGwAAIA/BQt0ZW1wZXJhdHVyZWZmZj8ACgVvY2VhbgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACgZwbGFpbnMFCGRvd25mYWxszczMPgULdGVtcGVyYXR1cmXNzEw/AAobcmVkd29vZF90YWlnYV9oaWxsc19tdXRhdGVkBQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlmpmZPgAKFXJlZHdvb2RfdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgVyaXZlcgUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZQAAAD8ACg1yb29mZWRfZm9yZXN0BQhkb3duZmFsbM3MTD8FC3RlbXBlcmF0dXJlMzMzPwAKFXJvb2ZlZF9mb3Jlc3RfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZTMzMz8ACgdzYXZhbm5hBQhkb3duZmFsbAAAAAAFC3RlbXBlcmF0dXJlmpmZPwAKD3NhdmFubmFfbXV0YXRlZAUIZG93bmZhbGwAAAA/BQt0ZW1wZXJhdHVyZc3MjD8ACg9zYXZhbm5hX3BsYXRlYXUFCGRvd25mYWxsAAAAAAULdGVtcGVyYXR1cmUAAIA/AAoXc2F2YW5uYV9wbGF0ZWF1X211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAIA/AAoLc3RvbmVfYmVhY2gFCGRvd25mYWxsmpmZPgULdGVtcGVyYXR1cmXNzEw+AAoQc3VuZmxvd2VyX3BsYWlucwUIZG93bmZhbGzNzMw+BQt0ZW1wZXJhdHVyZc3MTD8ACglzd2FtcGxhbmQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoRc3dhbXBsYW5kX211dGF0ZWQFCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmXNzEw/AAoFdGFpZ2EFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoLdGFpZ2FfaGlsbHMFCGRvd25mYWxszcxMPwULdGVtcGVyYXR1cmUAAIA+AAoNdGFpZ2FfbXV0YXRlZAUIZG93bmZhbGzNzEw/BQt0ZW1wZXJhdHVyZQAAgD4ACgd0aGVfZW5kBQhkb3duZmFsbAAAAD8FC3RlbXBlcmF0dXJlAAAAPwAKCndhcm1fb2NlYW4FCGRvd25mYWxsAAAAPwULdGVtcGVyYXR1cmUAAAA/AAA=";
        let nbt = Value::read(
            &mut Bytes::from(base64ct::Base64::decode_vec(&biomes).unwrap()),
            &mut NetworkLittleEndian,
        )
        .unwrap();
        conn.write_packet(
            &BiomeDefinitionList {
                serialised_biome_definitions: nbt.into(),
            }
            .into(),
        )
        .await;

        expectancies.queue::<SetLocalPlayerAsInitialised>().await;
        conn.write_packet(&Packet::from(PlayStatus {
            status: PlayStatusType::PlayerSpawn,
        }))
        .await;

        conn.write_packet(&Packet::from(CreativeContent { items: vec![] }))
            .await;
        conn.flush().await?;

        let final_pk = SetLocalPlayerAsInitialised::try_from(reader.recv().await).unwrap();
        if final_pk.entity_runtime_id.0 != rid as u64 {
            conn.write_packet(&Packet::from(Disconnect {
                message: Some(format!("Entity runtime ID mismatch.",)),
            }))
            .await;
            conn.flush().await?;
            conn.close().await?;
        }
        println!("Done");
        Ok(())
    }
}
