use uuid::Uuid;
use base64ct::{Base64, Encoding};
use serde::{Serialize, Deserialize};
use zuri_proto::types::device::Device;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdentityData {
    #[serde(rename = "XUID")]
    pub xuid: Option<String>,

    pub identity: String,

    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ClientData {
    pub animated_image_data: Vec<SkinAnimation>,

    pub cape_data: String,

    pub cape_id: String,

    pub cape_image_height: i32,

    pub cape_image_width: i32,

    pub cape_on_classic_skin: bool,

    pub client_random_id: i64,

    pub current_input_mode: i32,

    pub default_input_mode: i32,

    pub device_model: String,

    #[serde(rename = "DeviceOS")]
    pub device_os: Device,

    pub device_id: String,

    pub game_version: String,

    pub gui_scale: i32,

    #[serde(rename = "IsEditorMode")]
    pub editor_mode: bool,

    pub language_code: String,

    pub persona_skin: bool,

    pub platform_offline_id: String,

    pub platform_online_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_user_id: Option<String>,

    pub premium_skin: bool,

    pub self_signed_id: String,

    pub server_address: String,

    pub skin_animation_data: String,

    pub skin_data: String,

    pub skin_geometry_data: String,

    pub skin_geometry_data_engine_version: String,

    pub skin_id: String,

    pub play_fab_id: String,

    pub skin_image_height: i32,

    pub skin_image_width: i32,

    pub skin_resource_patch: String,

    #[serde(rename = "SkinColor")]
    pub skin_colour: String,

    pub arm_size: String,

    pub persona_pieces: Vec<PersonaPiece>,

    #[serde(rename = "PieceTintColors")]
    pub piece_tint_colours: Vec<PersonaPieceTintColour>,

    pub third_party_name: String,

    pub third_party_name_only: bool,

    #[serde(rename = "UIProfile")]
    pub ui_profile: i32,

    pub trusted_skin: bool,
}

const DEFAULT_SKIN_GEOMETRY: &[u8] = include_bytes!("default_model.json");
const DEFAULT_RESOURCE_PATCH: &[u8] = include_bytes!("default_resource_patch.json");

impl Default for ClientData {
    fn default() -> Self {
        Self {
            animated_image_data: Vec::new(),
            cape_data: String::new(),
            cape_id: String::new(),
            cape_image_height: 0,
            cape_image_width: 0,
            cape_on_classic_skin: false,
            client_random_id: rand::random(),
            current_input_mode: 0,
            default_input_mode: 0,
            device_model: String::new(),
            device_os: Device::Android,
            device_id: Uuid::new_v4().to_string(),
            game_version: zuri_proto::CURRENT_VERSION.to_string(),
            gui_scale: 0,
            editor_mode: false,
            language_code: String::from("en_US"),
            persona_skin: false,
            platform_offline_id: String::new(),
            platform_online_id: String::new(),
            platform_user_id: None,
            premium_skin: false,
            self_signed_id: Uuid::new_v4().to_string(),
            server_address: String::new(),
            skin_animation_data: String::new(),
            skin_data: Base64::encode_string(&vec![0, 0, 0, 255].repeat(32 * 64)),
            skin_geometry_data: Base64::encode_string(DEFAULT_SKIN_GEOMETRY),
            skin_geometry_data_engine_version: String::new(),
            skin_id: Uuid::new_v4().to_string(),
            play_fab_id: String::new(),
            skin_image_height: 32,
            skin_image_width: 64,
            skin_resource_patch: Base64::encode_string(DEFAULT_RESOURCE_PATCH),
            skin_colour: String::new(),
            arm_size: String::new(),
            persona_pieces: Vec::new(),
            piece_tint_colours: Vec::new(),
            third_party_name: String::new(),
            third_party_name_only: false,
            ui_profile: 0,
            trusted_skin: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PersonaPiece {
    #[serde(rename = "IsDefault")]
    pub default: bool,

    pub pack_id: String,

    pub piece_id: String,

    pub piece_type: String,

    pub product_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PersonaPieceTintColour {
    #[serde(rename = "Colors")]
    pub colours: [String; 4],

    pub piece_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SkinAnimation {
    pub frames: f64,

    pub image: String,

    pub image_height: i32,

    pub image_width: i32,

    #[serde(rename = "Type")]
    pub animation_type: i32,

    pub animation_expression: i32,
}
