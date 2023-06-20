use crate::xbox::LiveTokenResponse;
use base64ct::Encoding;
use p384::ecdsa::SigningKey;
use p384::pkcs8::EncodePublicKey;
use serde::{Deserialize, Serialize};

/// The URL that an authentication request is made to to get an encoded JWT claim chain.
const MINECRAFT_AUTH_URL: &str = "https://multiplayer.minecraft.net/authentication";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChainRequest {
    identity_public_key: String,
}

/// Requests a fully processed Minecraft JWT chain using the XSTS token passed, and the ECDSA
/// private key of the client. This key will later be used to initialise encryption, and must be
/// saved for when packets need to be decrypted/encrypted.
pub fn request_minecraft_chain(
    xbl_token: LiveTokenResponse,
    client_version: String,
    key: &SigningKey,
) -> String {
    let chain_request = ChainRequest {
        identity_public_key: base64ct::Base64::encode_string(
            key.verifying_key().to_public_key_der().unwrap().as_bytes(),
        ),
    };

    ureq::post(MINECRAFT_AUTH_URL)
        .set("user-agent", "MCPE/Android")
        .set("client-version", &client_version)
        .set("authorization", &xbl_token.to_string())
        .send_json(chain_request)
        .unwrap()
        .into_string()
        .unwrap()
}
