use base64ct::Encoding;
use bytes::BufMut;
use chrono::Utc;
use oauth2::basic::BasicTokenResponse;
use oauth2::TokenResponse;
use p256::ecdsa::signature::hazmat::PrehashSigner;
use p256::ecdsa::signature::Signature;
use p256::ecdsa::SigningKey;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::SecretKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Requests an XBOX Live auth token using the passed Live token pair.
pub fn request_xbl_token(
    live_token: &BasicTokenResponse,
    relying_party: String,
) -> LiveTokenResponse {
    let key = SecretKey::random(&mut rand::thread_rng());
    obtain_xbl_token(&key, live_token, obtain_device_token(&key), relying_party)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct LiveTokenRequest {
    access_token: String,
    app_id: String,
    proof_key: ProofKey,
    relying_party: String,
    sandbox: String,
    site_name: String,
    #[serde(rename = "UseModernGamertag")]
    use_modern_username: bool,
    #[serde(rename = "deviceToken")]
    device_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LiveTokenResponse {
    pub device_token: String,
    pub title_token: TitleToken,
    pub user_token: UserToken,
    pub authorization_token: AuthorizationToken,
    pub web_page: String,
    pub sandbox: String,
    pub use_modern_gamertag: bool,
}

impl ToString for LiveTokenResponse {
    fn to_string(&self) -> String {
        format!(
            "XBL3.0 x={};{}",
            self.authorization_token
                .display_claims
                .xui
                .get(0)
                .unwrap()
                .uhs,
            self.authorization_token.token,
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TitleToken {
    pub display_claims: XtiDisplayClaims,
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct XtiDisplayClaims {
    pub xti: XtiDisplayClaim,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct XtiDisplayClaim {
    pub tid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserToken {
    pub display_claims: XuiDisplayClaims,
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct XuiDisplayClaims {
    pub xui: Vec<XuiDisplayClaim>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct XuiDisplayClaim {
    pub uhs: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthorizationToken {
    pub display_claims: XuiDisplayClaims,
    pub issue_instant: String,
    pub not_after: String,
    pub token: String,
}

fn obtain_xbl_token(
    key: &SecretKey,
    live_token: &BasicTokenResponse,
    device_token: String,
    relying_party: String,
) -> LiveTokenResponse {
    let token_request = LiveTokenRequest {
        access_token: format!("t={}", live_token.access_token().secret()),
        app_id: "0000000048183522".into(),
        proof_key: ProofKey::new(key),
        relying_party,
        sandbox: "RETAIL".into(),
        site_name: "user.auth.xboxlive.com".into(),
        use_modern_username: true,
        device_token,
    };

    let body = serde_json::to_vec(&token_request).unwrap();
    let mut request = ureq::post("https://sisu.xboxlive.com/authorize")
        .set("content-type", "application/json")
        .set("accept", "application/json")
        .set("x-xbl-contract-version", "1");

    let signature = sign(&request, &body, key.into());
    request = request.set("signature", &signature);

    request
        .send_bytes(&body)
        .unwrap()
        .into_json::<LiveTokenResponse>()
        .unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DeviceTokenRequest {
    properties: DeviceTokenProperties,
    relying_party: String,
    token_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DeviceTokenProperties {
    auth_method: String,
    device_type: String,
    id: String,
    proof_key: ProofKey,
    version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DeviceTokenResponse {
    token: String,
}

fn obtain_device_token(key: &SecretKey) -> String {
    let token_request = DeviceTokenRequest {
        relying_party: "http://auth.xboxlive.com".into(),
        token_type: "JWT".into(),
        properties: DeviceTokenProperties {
            auth_method: "ProofOfPossession".into(),
            device_type: "Android".into(),
            id: format!("{{{}}}", Uuid::new_v4()),
            proof_key: ProofKey::new(key),
            version: "10".into(),
        },
    };

    let body = serde_json::to_vec(&token_request).unwrap();
    let mut request = ureq::post("https://device.auth.xboxlive.com/device/authenticate")
        .set("content-type", "application/json")
        .set("accept", "application/json")
        .set("x-xbl-contract-version", "1");

    let signature = sign(&request, &body, key.into());
    request = request.set("signature", &signature);

    request
        .send_bytes(&body)
        .unwrap()
        .into_json::<DeviceTokenResponse>()
        .unwrap()
        .token
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
struct ProofKey {
    alg: String,
    crv: String,
    kty: String,
    #[serde(rename = "use")]
    u: String,
    x: String,
    y: String,
}

impl ProofKey {
    fn new(key: &SecretKey) -> Self {
        let point = key.public_key().to_encoded_point(false);
        Self {
            crv: "P-256".into(),
            alg: "ES256".into(),
            u: "sig".into(),
            kty: "EC".into(),
            x: base64ct::Base64UrlUnpadded::encode_string(point.x().unwrap().as_slice()),
            y: base64ct::Base64UrlUnpadded::encode_string(point.y().unwrap().as_slice()),
        }
    }
}

/// Signs the request body given to it using the given ECDSA P256 signing key. The signature is
/// returned as a base64 encoded string.
fn sign(request: &ureq::Request, body: &Vec<u8>, signing_key: SigningKey) -> String {
    // Windows specific timestamp. It has an offset from Unix time which must be accounted for.
    let timestamp = (Utc::now().timestamp() + 11644473600) * 10000000;

    let mut digest = Sha256::new();

    // The signature policy version (1) along with a null terminator.
    let mut encoded_header = Vec::new();
    encoded_header.put_i32(1);
    encoded_header.put_u8(0);

    // The Windows timestamp along with a null terminator.
    encoded_header.put_i64(timestamp);
    encoded_header.put_u8(0);

    // Apply the header to the digest.
    digest.update(&encoded_header);

    // The HTTP method along with a null terminator.
    digest.update(request.method().as_bytes());
    digest.update([0]);

    // The relative path of the request along with a null terminator.
    digest.update(request.request_url().unwrap().path());
    digest.update([0]);

    // The authorization header of the request along with a null terminator.
    digest.update(request.header("authorization").unwrap_or("").as_bytes());
    digest.update([0]);

    // The body of the request along with a null terminator.
    digest.update(body);
    digest.update([0]);

    // Sign the digest using the given ECDSA P256 signing key.
    let signature = signing_key.sign_prehash(&digest.finalize()).unwrap();

    // The signature policy version and timestamp are prepended to the signature.
    let mut signature_with_header = Vec::new();
    signature_with_header.put_i32(1);
    signature_with_header.put_i64(timestamp);
    signature_with_header.put_slice(signature.as_bytes());

    // The signature is finally base64 encoded and set as the signature header.
    base64ct::Base64::encode_string(&signature_with_header)
}
