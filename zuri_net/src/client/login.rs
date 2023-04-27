use std::sync::Arc;

use async_trait::async_trait;
use base64ct::{Base64, Base64Unpadded, Encoding};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use oauth2::basic::BasicTokenResponse;
use p384::ecdsa::VerifyingKey;
use p384::pkcs8::{DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use sha2::{Digest, Sha256};

use zuri_xbox::{minecraft, xbox};

use crate::chan::PkReceiver;
use crate::client::auth::{IdentityClaims, IdentityPublicKeyClaims, Request, SaltClaims};
use crate::client::data::{ClientData, IdentityData};
use crate::connection::*;
use crate::encryption::Encryption;
use crate::proto::packet::chunk_radius_updated::ChunkRadiusUpdated;
use crate::proto::packet::client_cache_status::ClientCacheStatus;
use crate::proto::packet::client_to_server_handshake::ClientToServerHandshake;
use crate::proto::packet::login::Login;
use crate::proto::packet::network_settings::NetworkSettings;
use crate::proto::packet::play_status::{PlayStatus, PlayStatusType};
use crate::proto::packet::request_chunk_radius::RequestChunkRadius;
use crate::proto::packet::request_network_settings::RequestNetworkSettings;
use crate::proto::packet::resource_pack_client_response::ResourcePackClientResponse;
use crate::proto::packet::resource_pack_stack::ResourcePackStack;
use crate::proto::packet::resource_packs_info::ResourcePacksInfo;
use crate::proto::packet::server_to_client_handshake::ServerToClientHandshake;
use crate::proto::packet::set_local_player_as_initialised::SetLocalPlayerAsInitialised;
use crate::proto::packet::start_game::StartGame;
use crate::proto::packet::Packet;
use crate::proto::types::resource_pack::ResourcePackResponse;
use crate::proto::{CURRENT_PROTOCOL, CURRENT_VERSION};

pub struct LoginSequence<'a> {
    client_data: &'a ClientData,
    identity_data: &'a IdentityData,
    live_token: Option<BasicTokenResponse>,

    cache_chunks: bool,
    // TODO: Make a general GameData system.
}

#[async_trait]
impl<'a> Sequence<Result<(), ConnError>> for LoginSequence<'a> {
    async fn execute(
        self,
        mut reader: PkReceiver,
        conn: Arc<Connection>,
        expectancies: Arc<ExpectedPackets>,
    ) -> Result<(), ConnError> {
        // The first bit of the login sequence requires us to request the network settings the
        // server is using from the server. These dictate options for mostly compression, but also
        // various other things that aren't relevant to us.
        expectancies.queue::<NetworkSettings>().await;
        self.adapt_network_settings(&mut reader, &conn).await?;

        // Once we've received the server's network settings and adapted compression according to
        // the server's standards, we can actually send our login.
        expectancies.queue::<PlayStatus>().await;
        expectancies.queue::<ResourcePacksInfo>().await;
        expectancies.queue::<ServerToClientHandshake>().await;
        self.send_login(&conn).await?;

        // We'll either get one of two packets here; an encryption handshake, or a play status if
        // the server doesn't support encryption. We'll handle both cases here.
        match reader.recv().await {
            Packet::ServerToClientHandshake(handshake) => {
                // Adapt to encryption using the server's given JWT.
                self.adapt_encryption(&conn, String::from_utf8(handshake.jwt.to_vec()).unwrap())
                    .await?;

                // We can now expect a PlayStatus indicating a successful login.
                let play_status = PlayStatus::try_from(reader.recv().await).unwrap();
                if play_status.status != PlayStatusType::LoginSuccess {
                    panic!("login failed"); // TODO: proper error handling.
                }
            }
            Packet::PlayStatus(play_status) => {
                if play_status.status != PlayStatusType::LoginSuccess {
                    panic!("login failed"); // TODO: proper error handling.
                }

                // We didn't have encryption enabled on the server, so we'll still be expecting a
                // handshake from the server. We'll just retract our expectation for that.
                expectancies.retract::<ServerToClientHandshake>().await;
            }
            _ => todo!(), // todo
        }

        // Notify the server of our client cache status. Nintendo Switch clients don't properly
        // support this for whatever reason, so servers have to account for it.
        conn.write_packet(
            &mut ClientCacheStatus {
                enabled: self.cache_chunks,
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        // The server has entered the resource pack phase. We can expect a ResourcePacksInfo packet
        // containing all the information about the resource packs the server is using.
        expectancies.queue::<StartGame>().await;
        expectancies.queue::<ResourcePackStack>().await;
        self.download_resource_packs(&mut reader, &conn).await?;

        // The StartGame packet contains our runtime ID which we need later in the sequence.
        let mut rid = 0;

        // We can now expect a StartGame packet, which contains all the information we need to start
        // the game locally. Right now, all we need is the runtime ID, but in the future we'll need
        // to store a lot more information.
        expectancies.queue::<PlayStatus>().await;
        expectancies.queue::<ChunkRadiusUpdated>().await;
        self.await_start_game(&mut reader, &conn, &mut rid).await?;

        // We'll now need both the chunk radius and the play status to be sent to us. Once both are
        // sent, we can notify the server that we're ready to start playing.
        while expectancies.expecting_any().await {
            match reader.recv().await {
                Packet::ChunkRadiusUpdated(_) => {
                    // TODO: Store the chunk radius we received.
                }
                Packet::PlayStatus(play_status) => {
                    if play_status.status != PlayStatusType::PlayerSpawn {
                        panic!("login failed"); // TODO: proper error handling.
                    }
                }
                _ => unreachable!(),
            }
        }

        // Notify the server that we're initialized.
        conn.write_packet(
            &mut SetLocalPlayerAsInitialised {
                entity_runtime_id: rid.into(),
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        // We're done!
        Ok(())
    }
}

impl<'a> LoginSequence<'a> {
    pub fn new(
        client_data: &'a ClientData,
        identity_data: &'a IdentityData,
        live_token: Option<BasicTokenResponse>,
        cache_chunks: bool,
    ) -> Self {
        Self {
            live_token,
            client_data,
            identity_data,
            cache_chunks,
        }
    }

    async fn adapt_network_settings(
        &self,
        reader: &mut PkReceiver,
        conn: &Connection,
    ) -> Result<(), ConnError> {
        conn.write_packet(
            &mut RequestNetworkSettings {
                client_protocol: CURRENT_PROTOCOL.into(),
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        let pk = NetworkSettings::try_from(reader.recv().await).unwrap();
        conn.set_compression(pk.compression_algorithm).await;

        Ok(())
    }

    async fn adapt_encryption(&self, conn: &Connection, jwt: String) -> Result<(), ConnError> {
        let header = jsonwebtoken::decode_header(&jwt).unwrap();

        let mut validation = Validation::new(header.alg);
        validation.insecure_disable_signature_validation(); // TODO: This definitely is not right.
        validation.set_required_spec_claims::<String>(&[]);

        let unparsed_key = Base64::decode_vec(&header.x5u.unwrap()).unwrap();
        let server_verifying_key = VerifyingKey::from_public_key_der(&unparsed_key).unwrap();

        let token = jsonwebtoken::decode::<SaltClaims>(
            &jwt,
            &DecodingKey::from_ec_der(&unparsed_key),
            &validation,
        )
        .unwrap();

        let salt = Base64Unpadded::decode_vec(token.claims.salt.trim_end_matches('=')).unwrap();

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

    async fn send_login(&self, conn: &Connection) -> Result<(), ConnError> {
        let mut request = if self.live_token.is_none() {
            self.encode_offline_request(conn)?
        } else {
            self.encode_online_request(
                conn,
                minecraft::request_minecraft_chain(
                    xbox::request_xbl_token(
                        self.live_token.as_ref().unwrap(),
                        "https://multiplayer.minecraft.net/".into(),
                    ),
                    CURRENT_VERSION.into(),
                    conn.signing_key(),
                ),
            )?
        };

        conn.write_packet(
            &mut Login {
                client_protocol: CURRENT_PROTOCOL.into(),
                connection_request: request.encode().into(),
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        Ok(())
    }

    async fn download_resource_packs(
        &self,
        reader: &mut PkReceiver,
        conn: &Connection,
    ) -> Result<(), ConnError> {
        ResourcePacksInfo::try_from(reader.recv().await).unwrap();

        // TODO: Implement proper resource pack downloading

        conn.write_packet(
            &mut ResourcePackClientResponse {
                response: ResourcePackResponse::AllPacksDownloaded,
                packs_to_download: Vec::new(),
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        ResourcePackStack::try_from(reader.recv().await).unwrap();

        conn.write_packet(
            &mut ResourcePackClientResponse {
                response: ResourcePackResponse::Completed,
                packs_to_download: Vec::new(),
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        Ok(())
    }

    async fn await_start_game(
        &self,
        reader: &mut PkReceiver,
        conn: &Connection,
        rid: &mut u64,
    ) -> Result<(), ConnError> {
        let start_game = StartGame::try_from(reader.recv().await).unwrap();

        // TODO: Store rest of game data and update shield ID.
        *rid = start_game.entity_runtime_id.into();

        // We need to request a sample radius of chunks around the player in order for the server
        // to allow us to spawn in. This is a bit of a hack, but it's necessary.
        conn.write_packet(
            &mut RequestChunkRadius {
                chunk_radius: 16.into(),
                max_chunk_radius: 16.into(),
            }
            .into(),
        )
        .await;
        conn.flush().await?;

        Ok(())
    }

    fn encode_offline_request(&self, conn: &Connection) -> Result<Request, ConnError> {
        // TODO: CLEAN UP
        let signing_key = conn.signing_key();
        let encoding_key = EncodingKey::from_ec_der(signing_key.to_pkcs8_der().unwrap().as_bytes());
        let identity_public_key = Base64::encode_string(
            signing_key
                .verifying_key()
                .to_public_key_der()
                .unwrap()
                .as_bytes(),
        );

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.x5u = Some(identity_public_key.clone());
        header.typ = None;

        let now = Utc::now();
        let identity_jwt = jsonwebtoken::encode(
            &header,
            &IdentityClaims {
                expiration: (now + Duration::hours(6)).timestamp() as u64,
                not_before: (now - Duration::hours(6)).timestamp() as u64,
                identity_data: self.identity_data.clone(),
                identity_public_key: identity_public_key.clone(),
            },
            &encoding_key,
        )
        .unwrap();

        let data_jwt = jsonwebtoken::encode(&header, &self.client_data, &encoding_key).unwrap();

        Ok(Request {
            chain: vec![identity_jwt.as_str().to_string()],
            token: data_jwt.as_str().to_string(),
        })
    }

    fn encode_online_request(
        &self,
        conn: &Connection,
        signed_chain: String,
    ) -> Result<Request, ConnError> {
        let mut request: Request = serde_json::from_str(&signed_chain).unwrap();

        // TODO: CLEAN UP
        let signing_key = conn.signing_key();
        let encoding_key = EncodingKey::from_ec_der(signing_key.to_pkcs8_der().unwrap().as_bytes());
        let identity_public_key = Base64::encode_string(
            signing_key
                .verifying_key()
                .to_public_key_der()
                .unwrap()
                .as_bytes(),
        );

        let mut header = jsonwebtoken::Header::new(Algorithm::ES384);
        header.x5u = Some(identity_public_key.clone());
        header.typ = None;

        let now = Utc::now();
        let identity_public_key_jwt = jsonwebtoken::encode(
            &header,
            &IdentityPublicKeyClaims {
                expiration: (now + Duration::hours(6)).timestamp() as u64,
                not_before: (now - Duration::hours(6)).timestamp() as u64,
                identity_public_key: jsonwebtoken::decode_header(&request.chain[0])
                    .unwrap()
                    .x5u
                    .unwrap(),
                certificate_authority: Some(true),
            },
            &encoding_key,
        )
        .unwrap();

        let data_jwt = jsonwebtoken::encode(&header, &self.client_data, &encoding_key).unwrap();

        // prepend our identity public key JWT to the chain
        request.chain.insert(0, identity_public_key_jwt);
        request.token = data_jwt.as_str().to_string();

        Ok(request)
    }
}
