use base64ct::Encoding;
use p384::ecdsa::SigningKey;
use p384::pkcs8::EncodePublicKey;
use serde::{Deserialize, Serialize};
use crate::xbox::LiveTokenResponse;

/// The URL that an authentication request is made to to get an encoded JWT claim chain.
const MINECRAFT_AUTH_URL: &str = "https://multiplayer.minecraft.net/authentication";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MinecraftChainRequest {
    identity_public_key: String,
}

/// Requests a fully processed Minecraft JWT chain using the XSTS token passed, and the ECDSA
/// private key of the client. This key will later be used to initialise encryption, and must be
/// saved for when packets need to be decrypted/encrypted.
pub fn request_minecraft_chain(xbl_token: LiveTokenResponse, client_version: String, key: &SigningKey) -> String {
    let chain_request = MinecraftChainRequest {
        identity_public_key: base64ct::Base64::encode_string(
            key.verifying_key().to_public_key_der().unwrap().as_bytes(),
        ),
    };

    ureq::post(MINECRAFT_AUTH_URL)
        .set("user-agent", "MCPE/Android")
        .set("client-version", &client_version)
        .set("authorization", &xbl_token.to_string())
        .send_json(&chain_request).unwrap().into_string().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{live, xbox};
    use super::*;

    #[test]
    fn test_chain() {
        let details = live::start_device_auth().unwrap();

        println!(
            "Authenticate at {} using the code: {}",
            details.verification_uri().to_string(),
            details.user_code().secret().to_string()
        );

        let live_token = live::await_device_auth(details).unwrap();
        println!("Authenticated.");

        let xbl_token = xbox::request_xbl_token(
            live_token,
            "https://multiplayer.minecraft.net/".into(),
        );
        println!("Received XBL token ({}).", xbl_token.to_string());

        let key = SigningKey::random(&mut rand::thread_rng());
        let chain = request_minecraft_chain(xbl_token, "1.19.50".into(), &key);
        println!("Received Minecraft chain ({}).", chain);
    }
}
