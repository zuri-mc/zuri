use crate::chan::PkReceiver;
use crate::client::auth;
use crate::compression::Compression;
use crate::connection::{ConnError, Connection, ExpectedPackets, Sequence};
use crate::proto;
use crate::proto::packet::client_cache_status::ClientCacheStatus;
use crate::proto::packet::disconnect::Disconnect;
use crate::proto::packet::login::Login;
use crate::proto::packet::network_settings::NetworkSettings;
use crate::proto::packet::play_status::{PlayStatus, PlayStatusType};
use crate::proto::packet::request_network_settings::RequestNetworkSettings;
use crate::proto::packet::Packet;
use async_trait::async_trait;
use rand::random;
use uuid::Uuid;
use zuri_nbt::Value;
use crate::proto::io::NBT;
use crate::proto::packet::chunk_radius_updated::ChunkRadiusUpdated;
use crate::proto::packet::request_chunk_radius::RequestChunkRadius;
use crate::proto::packet::resource_pack_client_response::ResourcePackClientResponse;
use crate::proto::packet::resource_packs_info::ResourcePacksInfo;
use crate::proto::packet::set_local_player_as_initialised::SetLocalPlayerAsInitialised;
use crate::proto::packet::start_game::{ChatRestrictionLevel, EducationEditionRegion, GamePublishSetting, SpawnBiomeType, StartGame};
use crate::proto::types::education::EducationSharedResourceURI;
use crate::proto::types::item_stack::ItemEntry;
use crate::proto::types::player::PlayerMovementSettings;
use crate::proto::types::resource_pack::ResourcePackResponse;
use crate::proto::types::world::{Difficulty, Dimension, GameType, Generator, PermissionLevel};

pub struct LoginSequence {}

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
                compression_threshold: 512,
                compression_algorithm: Compression::Deflate,
                client_throttle: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            }))
            .await;
            conn.flush().await?;
            conn.set_compression(Compression::Deflate).await;
        }

        // Phase 2: Login.
        {
            let login = Login::try_from(reader.recv().await).unwrap();
            let request: serde_json::Result<auth::Request> =
                serde_json::from_slice(login.connection_request.as_ref());
            //if let Err(err) = request {
            //    conn.write_packet(&Packet::from(Disconnect {
            //        message: Some(format!("Malformed login request.")),
            //    }))
            //    .await;
            //    conn.flush().await?;
            //    conn.close().await?;
            //    eprintln!("Error: {}", err);
            //    return Ok(()); // todo: return actual error
            //}
            // todo: verify login request
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
        println!("Resource pack response: {:?}", resp);
        if resp.response != ResourcePackResponse::AllPacksDownloaded {
            todo!()
        }

        expectancies.queue::<RequestChunkRadius>().await;

        let rid = 0i64;
        conn.write_packet(&Packet::from(StartGame {
            entity_unique_id: rid.into(),
            entity_runtime_id: (rid as u64).into(),
            player_game_mode: GameType::Default,
            player_position: Default::default(),
            pitch: 0.0,
            yaw: 0.0,
            world_seed: 0,
            spawn_biome_type: SpawnBiomeType::Default,
            user_defined_biome_name: "".to_string(),
            dimension: Dimension::Overworld,
            generator: Generator::Overworld,
            world_game_mode: GameType::Survival,
            difficulty: Difficulty::Peaceful,
            world_spawn: Default::default(),
            achievements_disabled: true,
            editor_world: false,
            day_cycle_lock_time: Default::default(),
            education_edition_offer: EducationEditionRegion::RestOfWorld,
            education_features_enabled: false,
            education_product_id: Uuid::from_u128(random()).to_string(),
            rain_level: 0.0,
            lightning_level: 0.0,
            confirmed_platform_locked_content: false,
            multi_player_game: true,
            lan_broadcast_enabled: true,
            xbl_broadcast_mode: GamePublishSetting::Public,
            platform_broadcast_mode: GamePublishSetting::Public,
            commands_enabled: true,
            texture_pack_required: false,
            game_rules: vec![],
            experiments: vec![],
            experiments_previously_toggled: false,
            bonus_chest_enabled: false,
            start_with_map_enabled: false,
            player_permissions: PermissionLevel::Member,
            server_chunk_tick_radius: 10,
            has_locked_behaviour_pack: false,
            has_locked_texture_pack: false,
            from_locked_world_template: false,
            msa_gamer_tags_only: false,
            from_world_template: false,
            world_template_settings_locked: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            base_game_version: proto::CURRENT_VERSION.to_string(),
            limited_world_width: 0,
            limited_world_depth: 0,
            new_nether: true,
            education_shared_resource_uri: EducationSharedResourceURI { button_name: "".to_string(), link_uri: "".to_string() },
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
            items: vec![ItemEntry {
                name: "minecraft:stick".to_string(),
                runtime_id: 1,
                component_based: false,
            }],
            multi_player_correlation_id: Uuid::from_u128(random()).to_string(),
            server_authoritative_inventory: false,
            game_version: proto::CURRENT_VERSION.to_string(),
            property_data: NBT::from(Value::Compound(Default::default())),
            server_block_state_checksum: 0,
            world_template_id: Default::default(),
            client_side_generation: false,
        }))
            .await;
        conn.flush().await?;

        let req_chunk_radius = RequestChunkRadius::try_from(reader.recv().await).unwrap();

        conn.write_packet(&Packet::from(ChunkRadiusUpdated {
            chunk_radius: req_chunk_radius.chunk_radius.0.min(10).into(),
        }))
            .await;
        conn.flush().await?;

        expectancies.queue::<SetLocalPlayerAsInitialised>().await;
        conn.write_packet(&Packet::from(PlayStatus {
            status: PlayStatusType::PlayerSpawn,
        }))
            .await;
        conn.flush().await?;

        let final_pk = SetLocalPlayerAsInitialised::try_from(reader.recv().await).unwrap();
        if final_pk.entity_runtime_id.0 != rid as u64 {
            conn.write_packet(&Packet::from(Disconnect {
                message: Some(format!(
                    "Entity runtime ID mismatch.",
                )),
            }))
                .await;
            conn.flush().await?;
            conn.close().await?;
        }
        println!("Done");
        Ok(())
    }
}
