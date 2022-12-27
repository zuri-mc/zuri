use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use base64ct::{Base64, Base64Unpadded, Encoding};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use oauth2::basic::BasicTokenResponse;
use p384::ecdsa::VerifyingKey;
use p384::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use sha2::{Sha256, Digest};
use tokio::sync::mpsc::Receiver;
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

pub struct LoginSequence<'a> {
    client_data: &'a ClientData,
    identity_data: &'a IdentityData,
    live_token: Option<BasicTokenResponse>,

    cache_chunks: bool,
    // TODO: Make a general GameData system.
}

#[async_trait]
impl<'a> Sequence<()> for LoginSequence<'a> {
    async fn execute(self, mut reader: Receiver<Packet>, writer: SequenceConn) -> Result<(), ()> {
        println!("[{}:{}] Requesting network settings...", file!(), line!());
        self.adapt_network_settings(&mut reader, &writer).await.unwrap();
        println!("[{}:{}] Adapted to network settings, sending login...", file!(), line!());
        self.send_login(&mut reader, &writer).await.unwrap();
        println!("[{}:{}] Sent login, waiting for next step...", file!(), line!());

        match reader.recv().await.unwrap() {
            Packet::ServerToClientHandshake(handshake) => {
                println!("[{}:{}] Received server to client handshake, adapting encryption...", file!(), line!());
                self.adapt_encryption(
                    &mut reader,
                    &writer,
                    String::from_utf8(handshake.jwt.to_vec()).unwrap(),
                ).await.unwrap();
                println!("[{}:{}] Adapted encryption, awaiting successful login...", file!(), line!());

                let play_status = PlayStatus::try_from(
                    reader.recv().await.unwrap(),
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
        writer.write_packet( ClientCacheStatus { enabled: self.cache_chunks }.into()).await.unwrap();
        println!("[{}:{}] Sent client cache status, awaiting resource packs...", file!(), line!());

        self.download_resource_packs(&mut reader, &writer).await.unwrap();
        println!("[{}:{}] Downloaded resource packs, awaiting start game...", file!(), line!());

        // The start game packet contains our runtime ID which we need later in the sequence. In the
        // future, we should really have a generalized game data, but for now we'll just store it in
        // a local variable.
        let mut rid = 0;
        self.await_start_game(&mut reader, &writer, &mut rid).await.unwrap();
        println!("[{}:{}] Received start game and sent chunk radius.", file!(), line!());
        println!("[{}:{}] Sent request radius, awaiting response(s)...", file!(), line!());

        // TODO: FIX THE BELOW OH MY GOD WE'RE SO CLOSE
        // // We receive two packets here, ChunkRadiusUpdated and PlayStatus. The order in which these
        // // come in is not guaranteed, so we need to handle both cases.
        // let mut received_play_status = false;
        // let mut received_chunk_radius = false;
        // while !received_chunk_radius || !received_play_status {
        //     match conn.read_next_packet().await.unwrap() {
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
        writer.write_packet(SetLocalPlayerAsInitialised {
            entity_runtime_id: rid,
        }.into()).await.unwrap();

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

    async fn adapt_network_settings(&self, reader: &mut Receiver<Packet>, writer: &SequenceConn) -> Result<(), ConnError> {
        writer.write_packet(RequestNetworkSettings {
            client_protocol: CURRENT_PROTOCOL,
        }.into()).await?;

        let pk = NetworkSettings::try_from(reader.recv().await.unwrap()).unwrap();
        writer.set_compression(pk.compression_algorithm.into()).await;

        Ok(())
    }

    async fn adapt_encryption(&self, reader: &mut Receiver<Packet>, writer: &SequenceConn, jwt: String) -> Result<(), ConnError> {
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

        let signing_key = writer.signing_key().await;
        let unsalted_secret = p384::ecdh::diffie_hellman(
            signing_key.as_nonzero_scalar(),
            server_verifying_key.as_affine(),
        );

        let mut digest = Sha256::new();
        digest.update(&salt);
        digest.update(&unsalted_secret.raw_secret_bytes());

        let shared_secret = digest.finalize().to_vec();

        writer.set_encryption(Encryption::new(shared_secret)).await;

        writer.write_packet(ClientToServerHandshake.into()).await?;
        Ok(())
    }

    async fn send_login(&self, reader: &mut Receiver<Packet>, writer: &SequenceConn) -> Result<(), ConnError> {
        let mut request = if self.live_token.is_none() {
            self.encode_offline_request(writer).await?
        } else {
            self.encode_online_request(writer, minecraft::request_minecraft_chain(
                xbox::request_xbl_token(
                    self.live_token.as_ref().unwrap(),
                    "https://multiplayer.minecraft.net/".into(),
                ),
                CURRENT_VERSION.into(),
                &writer.signing_key().await,
            )).await?
        };

        let signing_key = writer.signing_key().await;
        let encoding_key = EncodingKey::from_ec_der(
            signing_key.to_pkcs8_der().unwrap().as_bytes(),
        );
        let identity_public_key = Base64::encode_string(
            signing_key.verifying_key().to_public_key_der().unwrap().as_bytes(),
        );

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.x5u = Some(identity_public_key.clone());
        header.typ = None;

        writer.write_packet(Login {
            client_protocol: CURRENT_PROTOCOL,
            connection_request: request.encode().into(),
        }.into()).await?;

        Ok(())
    }

    async fn download_resource_packs(&self, reader: &mut Receiver<Packet>, writer: &SequenceConn) -> Result<(), ConnError> {
        ResourcePacksInfo::try_from(
            reader.recv().await.unwrap(),
        ).unwrap();

        // TODO: Implement proper resource pack downloading

        writer.write_packet(ResourcePackClientResponse {
            response: ResourcePackResponse::Completed,
            packs_to_download: Vec::new(),
        }.into()).await?;

        Ok(())
    }

    async fn await_start_game(&self, reader: &mut Receiver<Packet>, writer: &SequenceConn, rid: &mut u64) -> Result<(), ConnError> {
        let start_game = StartGame::try_from(
            reader.recv().await.unwrap(),
        ).unwrap();

        // TODO: Store rest of game data and update shield ID.
        *rid = start_game.entity_runtime_id;

        // We need to request a sample radius of chunks around the player in order for the server
        // to allow us to spawn in. This is a bit of a hack, but it's necessary.
        writer.write_packet(RequestChunkRadius { chunk_radius: 16 }.into()).await?;
        Ok(())
    }

    async fn encode_offline_request(&self, writer: &SequenceConn) -> Result<Request, ConnError> {
        // TODO: CLEAN UP
        let signing_key = writer.signing_key().await;
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

    async fn encode_online_request(&self, writer: &SequenceConn, signed_chain: String) -> Result<Request, ConnError> {
        dbg!(signed_chain.clone());
        let mut request: Request = serde_json::from_str(&signed_chain).unwrap();

        // TODO: CLEAN UP
        let signing_key = writer.signing_key().await;
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
