use std::ops::Sub;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use base64ct::{Base64, Encoding};
use jsonwebtoken::{Algorithm, EncodingKey};
use p384::pkcs8::{EncodePrivateKey, EncodePublicKey};
use tokio::sync::Mutex;
use zuri_proto::CURRENT_PROTOCOL;
use zuri_proto::packet::login::Login;
use zuri_proto::packet::network_settings::NetworkSettings;
use zuri_proto::packet::Packet;
use zuri_proto::packet::request_network_settings::RequestNetworkSettings;

use crate::connection::*;
use crate::auth::{IdentityClaims, Request};
use crate::data::{ClientData, IdentityData};

pub struct LoginSequence<'a> {
    client_data: &'a ClientData,
    identity_data: &'a IdentityData,
}

#[async_trait]
impl<'a> Sequence<()> for LoginSequence<'a> {
    async fn execute(self, conn_mu: &Mutex<Connection>) -> Result<(), ()> {
        let mut conn = conn_mu.lock().await;
        self.adapt_network_settings(&mut conn).await.unwrap();
        self.send_login(&mut conn).await.unwrap();

        match conn.read_next_packet().await? {
            Packet::ServerToClientHandshake(pk) => {
                self.adapt_encryption(&mut conn, pk).await.unwrap();
                self.await_login_success(&mut conn).await.unwrap();
                received_handshake = true;
            },
            Packet::PlayStatus(pk) => {
                self.await_login_success(&mut conn).await.unwrap();
            }
            _ => return Err(()), // todo
        }

        Ok(())
    }
}

impl<'a> LoginSequence<'a> {
    pub fn new(client_data: &'a ClientData, identity_data: &'a IdentityData) -> Self {
        Self {
            client_data,
            identity_data,
        }
    }

    async fn adapt_network_settings(&self, conn: &mut Connection) -> Result<(), ConnError> {
        conn.write_packet(&mut RequestNetworkSettings {
            client_protocol: CURRENT_PROTOCOL,
        }.into());
        conn.flush().await?;

        let pk = NetworkSettings::try_from(conn.read_next_packet().await?).unwrap();
        conn.set_compression(pk.compression_algorithm.into());

        Ok(())
    }

    async fn adapt_encryption(&self, conn: &mut Connection) -> Result<(), ConnError> {

    }

    async fn send_login(&self, conn: &mut Connection) -> Result<(), ConnError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

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

        let identity_jwt = jsonwebtoken::encode(&header, &IdentityClaims {
            expiration: (now + Duration::from_secs(6 * 60 * 60)).as_secs(),
            not_before: (now - Duration::from_secs(6 * 60 * 60)).as_secs(),
            identity_data: self.identity_data.clone(),
            identity_public_key: identity_public_key.clone(),
        }, &encoding_key).unwrap();

        let data_jwt = jsonwebtoken::encode(
            &header, &self.client_data, &encoding_key,
        ).unwrap();

        conn.write_packet(&mut Login {
            client_protocol: CURRENT_PROTOCOL,
            connection_request: Request {
                chain: vec![identity_jwt.as_str().to_string()],
                token: data_jwt.as_str().to_string(),
            }.encode().into(),
        }.into());
        conn.flush().await?;

        Ok(())
    }
}
