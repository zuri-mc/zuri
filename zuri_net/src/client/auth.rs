use bytes::BufMut;
use serde::{Deserialize, Serialize};

use crate::client::data::IdentityData;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityClaims {
    #[serde(rename = "exp")]
    pub expiration: u64,

    #[serde(rename = "nbf")]
    pub not_before: u64,

    #[serde(rename = "extraData")]
    pub identity_data: IdentityData,

    #[serde(rename = "identityPublicKey")]
    pub identity_public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPublicKeyClaims {
    #[serde(
        rename = "certificateAuthority",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_authority: Option<bool>,

    #[serde(rename = "exp")]
    pub expiration: u64,

    #[serde(rename = "identityPublicKey")]
    pub identity_public_key: String,

    #[serde(rename = "nbf")]
    pub not_before: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaltClaims {
    pub salt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "chain")]
    pub chain: Vec<String>,

    #[serde(skip_serializing, skip_deserializing)]
    pub token: String,
}

impl Request {
    pub fn encode(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();

        let chain = serde_json::to_string(self).unwrap();

        buf.put_i32_le(chain.len() as i32);
        buf.put(chain.as_bytes());

        buf.put_i32_le(self.token.len() as i32);
        buf.put(self.token.as_bytes());

        buf
    }
}
