use bytes::Bytes;
use zuri_net_derive::proto;

#[proto]
#[derive(Debug, Default, Clone)]
pub struct Skin {
    pub skin_id: String,
    pub play_fab_id: String,
    pub skin_resource_patch: Bytes,
    pub skin_image_width: u32,
    pub skin_image_height: u32,
    pub skin_data: Bytes,
    #[len_type(u32)]
    pub animations: Vec<SkinAnimation>,
    pub cape_image_width: u32,
    pub cape_image_height: u32,
    pub cape_data: Bytes,
    pub skin_geometry: Bytes,
    pub geometry_data_engine_version: Bytes,
    pub animation_data: Bytes,
    pub cape_id: String,
    pub full_id: String,
    pub arm_size: String,
    pub skin_colour: String,
    #[len_type(u32)]
    pub persona_pieces: Vec<PersonaPiece>,
    #[len_type(u32)]
    pub piece_tint_colours: Vec<PersonaPieceTintColour>,
    pub premium_skin: bool,
    pub persona_skin: bool,
    pub persona_cape_on_classic_skin: bool,
    pub primary_user: bool,
    #[skip]
    pub trusted: bool,
    /// Specifies if the skin should override the player's skin that is equipped client-side. When
    /// false, the client will reject the skin and continue to use the skin that the player has
    /// equipped.
    pub override_appearance: bool,
}

#[proto]
#[derive(Debug, Clone)]
pub struct SkinAnimation {
    pub image_width: u32,
    pub image_height: u32,
    pub image_data: Bytes,
    pub animation_type: u32,
    pub frame_count: f32,
    pub expression_type: u32,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PersonaPiece {
    pub piece_id: String,
    pub piece_type: String,
    pub pack_id: String,
    pub default: bool,
    pub product_id: String,
}

#[proto]
#[derive(Debug, Clone)]
pub struct PersonaPieceTintColour {
    pub piece_type: String,
    #[len_type(u32)]
    pub colours: Vec<String>,
}
