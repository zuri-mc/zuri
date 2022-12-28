use std::sync::Arc;
use async_trait::async_trait;
use base64ct::{Base64, Base64Unpadded, Encoding};
use chrono::{Duration, Utc};
use crossbeam::channel::Receiver;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use oauth2::basic::BasicTokenResponse;
use p384::ecdsa::VerifyingKey;
use p384::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use sha2::{Sha256, Digest};
use tokio::sync::Mutex;
use zuri_xbox::{minecraft, xbox};
use crate::proto::{CURRENT_PROTOCOL, CURRENT_VERSION};
use crate::proto::packet::client_cache_status::ClientCacheStatus;
use crate::proto::packet::client_to_server_handshake::ClientToServerHandshake;
use crate::proto::packet::login::Login;
use crate::proto::packet::network_settings::NetworkSettings;
use crate::proto::packet::Packet;
use crate::proto::packet::play_status::{PlayStatus, PlayStatusType};
use crate::proto::packet::request_chunk_radius::RequestChunkRadius;
use crate::proto::packet::request_network_settings::RequestNetworkSettings;
use crate::proto::packet::resource_pack_client_response::ResourcePackClientResponse;
use crate::proto::packet::resource_packs_info::ResourcePacksInfo;
use crate::proto::packet::set_local_player_as_initialised::SetLocalPlayerAsInitialised;
use crate::proto::packet::start_game::StartGame;
use crate::proto::types::resource_pack::ResourcePackResponse;

use crate::connection::*;
use crate::client::auth::{IdentityClaims, IdentityPublicKeyClaims, Request, SaltClaims};
use crate::client::data::{ClientData, IdentityData};
use crate::encryption::Encryption;
use crate::proto::packet::server_to_client_handshake::ServerToClientHandshake;

pub struct LoginSequence<'a> {
    client_data: &'a ClientData,
    identity_data: &'a IdentityData,
    live_token: Option<BasicTokenResponse>,

    cache_chunks: bool,
    // TODO: Make a general GameData system.
}

#[async_trait]
impl<'a> Sequence<()> for LoginSequence<'a> {
    async fn execute(self, reader: Receiver<Packet>, conn: Arc<Connection>, expecter: Arc<ExpectedPackets>) -> Result<(), ()> {
        println!("[{}:{}] Requesting network settings...", file!(), line!());
        expecter.expect::<NetworkSettings>().await;
        self.adapt_network_settings(&reader, &conn).await.unwrap();
        println!("[{}:{}] Adapted to network settings, sending login...", file!(), line!());
        self.send_login(&reader, &conn).await.unwrap();
        println!("[{}:{}] Sent login, waiting for next step...", file!(), line!());

        expecter.expect::<ServerToClientHandshake>().await;
        expecter.expect::<PlayStatus>().await;
        expecter.expect::<ResourcePacksInfo>().await;
        match reader.recv().unwrap() {
            Packet::ServerToClientHandshake(handshake) => {
                println!("[{}:{}] Received server to client handshake, adapting encryption...", file!(), line!());
                self.adapt_encryption(
                    &reader,
                    &conn,
                    String::from_utf8(handshake.jwt.to_vec()).unwrap(),
                ).await.unwrap();
                println!("[{}:{}] Adapted encryption, awaiting successful login...", file!(), line!());

                let play_status = PlayStatus::try_from(
                    reader.recv().unwrap(),
                ).unwrap();
                if play_status.status != PlayStatusType::LoginSuccess {
                    panic!("login failed"); // TODO: proper error handling.
                }
                println!("[{}:{}] Login successful!", file!(), line!());
            }
            Packet::PlayStatus(play_status) => {
                if play_status.status != PlayStatusType::LoginSuccess {
                    panic!("login failed"); // TODO: proper error handling.
                }
                println!("[{}:{}] Login successful!", file!(), line!());
            }
            _ => return Err(()), // todo
        }

        println!("[{}:{}] Sending client cache status...", file!(), line!());
        conn.write_packet(&mut ClientCacheStatus { enabled: self.cache_chunks }.into()).await;
        conn.flush().await.unwrap();
        println!("[{}:{}] Sent client cache status, awaiting resource packs...", file!(), line!());

        self.download_resource_packs(&reader, &conn).await.unwrap();
        println!("[{}:{}] Downloaded resource packs, awaiting start game...", file!(), line!());

        // The start game packet contains our runtime ID which we need later in the sequence. In the
        // future, we should really have a generalized game data, but for now we'll just store it in
        // a local variable.
        let mut rid = 0;
        expecter.expect::<StartGame>().await;
        self.await_start_game(&reader, &conn, &mut rid).await.unwrap();
        println!("[{}:{}] Received start game and sent chunk radius.", file!(), line!());
        println!("[{}:{}] Sent request radius, awaiting response(s)...", file!(), line!());

        // TODO: FIX THE BELOW OH MY GOD WE'RE SO CLOSE
        // // We receive two packets here, ChunkRadiusUpdated and PlayStatus. The order in which these
        // // come in is not guaranteed, so we need to handle both cases.
        // let mut received_play_status = false;
        // let mut received_chunk_radius = false;
        // while !received_chunk_radius || !received_play_status {
        //     match reader.recv().unwrap() {
        //         Packet::ChunkRadiusUpdated(_) => {
        //             // TODO: Store the chunk radius we received.
        //             received_chunk_radius = true
        //         }
        //         Packet::PlayStatus(play_status) => {
        //             if play_status.status != PlayStatusType::PlayerSpawn {
        //                 panic!("login failed"); // TODO: proper error handling.
        //             }
        //             received_play_status = true;
        //         }
        //         _ => return Err(()), // todo
        //     }
        // }

        println!("[{}:{}] Received response(s), sending set local player as initialised...", file!(), line!());

        // Notify the server that we're initialized.
        conn.write_packet(&mut SetLocalPlayerAsInitialised {
            entity_runtime_id: rid,
        }.into()).await;
        conn.flush().await.unwrap();

        println!("[{}:{}] Login sequence complete!", file!(), line!());

        // We're done!
        Ok(())
    }
}

impl<'a> LoginSequence<'a> {
    pub fn new(client_data: &'a ClientData, identity_data: &'a IdentityData, live_token: Option<BasicTokenResponse>, cache_chunks: bool) -> Self {
        Self {
            live_token,
            client_data,
            identity_data,
            cache_chunks,
        }
    }

    async fn adapt_network_settings(&self, reader: &Receiver<Packet>, conn: &Connection) -> Result<(), ConnError> {
        conn.write_packet(&mut RequestNetworkSettings {
            client_protocol: CURRENT_PROTOCOL,
        }.into()).await;
        conn.flush().await?;

        let pk = NetworkSettings::try_from(reader.recv().unwrap()).unwrap();
        conn.set_compression(pk.compression_algorithm.into()).await;
        Ok(())
    }

    async fn adapt_encryption(&self, reader: &Receiver<Packet>, conn: &Connection, jwt: String) -> Result<(), ConnError> {
        let header = jsonwebtoken::decode_header(&jwt).unwrap();

        let mut validation = Validation::new(header.alg);
        validation.insecure_disable_signature_validation(); // TODO: This definitely is not right.
        validation.set_required_spec_claims::<String>(&[]);

        let unparsed_key = Base64::decode_vec(
            &header.x5u.unwrap(),
        ).unwrap();
        let server_verifying_key = VerifyingKey::from_public_key_der(
            &unparsed_key,
        ).unwrap();

        let token = jsonwebtoken::decode::<SaltClaims>(
            &jwt,
            &DecodingKey::from_ec_der(&unparsed_key), &validation,
        ).unwrap();

        let salt = Base64Unpadded::decode_vec(
            token.claims.salt.trim_end_matches('='),
        ).unwrap();

        let signing_key = conn.signing_key();
        let unsalted_secret = p384::ecdh::diffie_hellman(
            signing_key.as_nonzero_scalar(),
            server_verifying_key.as_affine(),
        );

        let mut digest = Sha256::new();
        digest.update(&salt);
        digest.update(&unsalted_secret.raw_secret_bytes());

        let shared_secret = digest.finalize().to_vec();

        conn.set_encryption(Encryption::new(shared_secret)).await;

        conn.write_packet(&mut ClientToServerHandshake.into()).await;
        conn.flush().await?;

        Ok(())
    }

    async fn send_login(&self, reader: &Receiver<Packet>, conn: &Connection) -> Result<(), ConnError> {
        let mut request = if self.live_token.is_none() {
            self.encode_offline_request(reader, conn)?
        } else {
            self.encode_online_request(reader, conn, minecraft::request_minecraft_chain(
                xbox::request_xbl_token(
                    self.live_token.as_ref().unwrap(),
                    "https://multiplayer.minecraft.net/".into(),
                ),
                CURRENT_VERSION.into(),
                conn.signing_key(),
            ))?
        };

        conn.write_packet(&mut Login {
            client_protocol: CURRENT_PROTOCOL,
            connection_request: request.encode().into(),
        }.into()).await;
        conn.flush().await?;

        Ok(())
    }

    async fn download_resource_packs(&self, reader: &Receiver<Packet>, conn: &Connection) -> Result<(), ConnError> {
        ResourcePacksInfo::try_from(
            reader.recv().unwrap(),
        ).unwrap();

        // TODO: Implement proper resource pack downloading

        conn.write_packet(&mut ResourcePackClientResponse {
            response: ResourcePackResponse::Completed,
            packs_to_download: Vec::new(),
        }.into()).await;
        conn.flush().await?;

        Ok(())
    }

    async fn await_start_game(&self, reader: &Receiver<Packet>, conn: &Connection, rid: &mut u64) -> Result<(), ConnError> {
        let start_game = StartGame::try_from(
            reader.recv().unwrap(),
        ).unwrap();

        // TODO: Store rest of game data and update shield ID.
        *rid = start_game.entity_runtime_id;

        // We need to request a sample radius of chunks around the player in order for the server
        // to allow us to spawn in. This is a bit of a hack, but it's necessary.
        conn.write_packet(&mut RequestChunkRadius { chunk_radius: 16 }.into()).await;
        conn.flush().await?;

        Ok(())
    }

    fn encode_offline_request(&self, reader: &Receiver<Packet>, conn: &Connection) -> Result<Request, ConnError> {
        // TODO: CLEAN UP
        let signing_key = conn.signing_key();
        let encoding_key = EncodingKey::from_ec_der(
            signing_key.to_pkcs8_der().unwrap().as_bytes(),
        );
        let identity_public_key = Base64::encode_string(
            signing_key.verifying_key().to_public_key_der().unwrap().as_bytes(),
        );

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.x5u = Some(identity_public_key.clone());
        header.typ = None;

        let now = Utc::now();
        let identity_jwt = jsonwebtoken::encode(&header, &IdentityClaims {
            expiration: (now + Duration::hours(6)).timestamp() as u64,
            not_before: (now - Duration::hours(6)).timestamp() as u64,
            identity_data: self.identity_data.clone(),
            identity_public_key: identity_public_key.clone(),
        }, &encoding_key).unwrap();

        let data_jwt = jsonwebtoken::encode(
            &header, &self.client_data, &encoding_key,
        ).unwrap();

        Ok(Request {
            chain: vec![identity_jwt.as_str().to_string()],
            token: data_jwt.as_str().to_string(),
        })
    }

    fn encode_online_request(&self, reader: &Receiver<Packet>, conn: &Connection, signed_chain: String) -> Result<Request, ConnError> {
        dbg!(signed_chain.clone());
        let mut request: Request = serde_json::from_str(&signed_chain).unwrap();

        // TODO: CLEAN UP
        let signing_key = conn.signing_key();
        let encoding_key = EncodingKey::from_ec_der(
            signing_key.to_pkcs8_der().unwrap().as_bytes(),
        );
        let identity_public_key = Base64::encode_string(
            signing_key.verifying_key().to_public_key_der().unwrap().as_bytes(),
        );

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.x5u = Some(identity_public_key.clone());
        header.typ = None;

        let now = Utc::now();
        let identity_public_key_jwt = jsonwebtoken::encode(&header, &IdentityPublicKeyClaims {
            expiration: (now + Duration::hours(6)).timestamp() as u64,
            not_before: (now - Duration::hours(6)).timestamp() as u64,
            identity_public_key: jsonwebtoken::decode_header(&request.chain[0]).unwrap().x5u.unwrap(),
            certificate_authority: Some(true),
        }, &encoding_key).unwrap();

        let data_jwt = jsonwebtoken::encode(
            &header, &self.client_data, &encoding_key,
        ).unwrap();

        // prepend our identity public key JWT to the chain
        request.chain.insert(0, identity_public_key_jwt);
        request.token = data_jwt.as_str().to_string();

        Ok(request)
    }
}
