use std::fs::File;
use oauth2::basic::{BasicClient, BasicErrorResponse, BasicTokenResponse};
use oauth2::devicecode::{DeviceCodeErrorResponse, StandardDeviceAuthorizationResponse};
use oauth2::ureq::Error as UReqError;
use oauth2::ureq::http_client;
use oauth2::{AuthType, AuthUrl, ClientId, DeviceAuthorizationUrl, RequestTokenError, Scope, TokenResponse, TokenUrl};

const DEVICE_AUTHORIZE_URL: &str = "https://login.live.com/oauth20_connect.srf";
const AUTHORIZE_URL: &str = "https://login.live.com/oauth20_authorize.srf";
const TOKEN_URL: &str = "https://login.live.com/oauth20_token.srf";

const CLIENT_ID: &str = "0000000048183522";
const SCOPE: &str = "service::user.auth.xboxlive.com::MBI_SSL";

/// Reads a Live token from the given path. If a token is not found, a new one is obtained and
/// saved to the given path. It takes in a detailer to provide information such as user codes.
pub fn read_or_obtain_token<F: FnOnce(&StandardDeviceAuthorizationResponse)>(
    path: String,
    detailer: F,
) -> BasicTokenResponse {
    let file = File::open(&path);
    if let Ok(file) = file {
        return serde_json::from_reader(file).unwrap();
    }

    // Start a new device code authentication flow.
    let details = start_device_auth().unwrap();
    detailer(&details);

    // Wait for the user to enter the code.
    let live_token = await_device_auth(details).unwrap();

    // Save the token to the given path.
    let file = File::create(&path).unwrap();
    serde_json::to_writer(file, &live_token).unwrap();

    live_token
}

/// Starts the device auth, retrieving a login URI for the user and a code the user needs to enter.
pub fn start_device_auth() -> Result<StandardDeviceAuthorizationResponse, RequestTokenError<UReqError, BasicErrorResponse>> {
    live_client()
        .exchange_device_code().unwrap()
        .add_scope(Scope::new(SCOPE.into()))
        .add_extra_param("response_type", "device_code")
        .request(http_client)
}

/// Polls the token endpoint until the user enters the code or the timeout is reached. If the
/// timeout is reached, the result will contain an error.
pub fn await_device_auth(auth: StandardDeviceAuthorizationResponse) -> Result<BasicTokenResponse, RequestTokenError<UReqError, DeviceCodeErrorResponse>> {
    live_client()
        .exchange_device_access_token(&auth)
        .request(http_client, std::thread::sleep, None)
}

/// Refreshes the BasicTokenResponse given to it. An error is returned if the refresh fails.
pub fn refresh_token(token: BasicTokenResponse) -> Result<BasicTokenResponse, RequestTokenError<UReqError, BasicErrorResponse>> {
    live_client()
        .exchange_refresh_token(token.refresh_token().unwrap())
        .request(http_client)
}

/// Returns a BasicClient ready to use for the Xbox Live API. It will panic if an error occurs.
fn live_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(CLIENT_ID.into()),
        None,
        AuthUrl::new(
            AUTHORIZE_URL.into(),
        ).unwrap(),
        Some(TokenUrl::new(
            TOKEN_URL.into(),
        ).unwrap()),
    ).set_auth_type(AuthType::RequestBody).set_device_authorization_url(DeviceAuthorizationUrl::new(
        DEVICE_AUTHORIZE_URL.into(),
    ).unwrap())
}
