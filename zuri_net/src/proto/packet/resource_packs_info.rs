use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

/// Sent by the server to inform the client on what resource packs the server has. It sends a list
/// of the resource packs it has and basic information on them like the version and description.
#[proto]
#[derive(Debug, Clone)]
pub struct ResourcePacksInfo {
    /// Specifies if the client must accept the texture packs the server has in order to join the
    /// server. If set to true, the client gets the option to either download the resource packs and
    /// join, or quit entirely. Behaviour packs never have to be downloaded.
    pub texture_pack_required: bool,
    /// Specifies if any of the resource packs contain scripts in them. If set to true, only clients
    /// that support scripts will be able to download them.
    pub has_scripts: bool,
    /// The use of this field is currently unknown.
    pub forcing_server_packs: bool,
    /// A list of behaviour packs that the client needs to download before joining the server. All
    /// of these behaviour packs will be applied together.
    #[len_type(u16)]
    pub behaviour_packs: Vec<BehaviourPackInfo>,
    /// A list of texture packs that the client needs to download before joining the server. The
    /// order of these texture packs is not relevant in this packet. It is however important in the
    /// ResourcePackStack packet.
    #[len_type(u16)]
    pub texture_packs: Vec<TexturePackInfo>,
    /// A list of texture packs to be downloaded over HTTP.
    #[len_type(VarU32)]
    pub pack_urls: Vec<PackUrl>,
}

/// Represents a resource pack served to the client over HTTP.
#[proto]
#[derive(Debug, Clone)]
pub struct PackUrl {
    /// The unique identifier for the resource pack.
    pub uuid: String,
    /// The URL from which the pack should be downloaded by the client.
    pub url: String,
}

/// Holds information about the behaviour pack such as its name, description and version.
#[proto]
#[derive(Debug, Clone)]
pub struct BehaviourPackInfo {
    /// The UUID of the behaviour pack. Each behaviour pack downloaded must have a different UUID in
    /// order for the client to be able to handle them properly.
    pub uuid: String,
    /// The version of the behaviour pack. The client will cache behaviour packs sent by the server
    /// as long as they carry the same version. Sending a behaviour pack with a different version
    /// than previously will force the client to re-download it.
    pub version: String,
    /// The total size in bytes that the behaviour pack occupies. This is the size of the compressed
    /// archive (zip) of the behaviour pack.
    pub size: u64,
    /// The key used to decrypt the behaviour pack if it is encrypted. This is generally the case
    /// for marketplace behaviour packs.
    pub content_key: String,
    /// The purpose of this field is currently unknown.
    pub sub_pack_name: String,
    /// The purpose of this field is currently unknown.
    pub content_identity: String,
    /// Specifies if the behaviour packs has any scripts in it. A client will only download the
    /// behaviour pack if it supports scripts, which, up to 1.11, only includes Windows 10.
    pub has_scripts: bool,
}

/// Holds information about the texture pack such as its name, description and version.
#[proto]
#[derive(Debug, Clone)]
pub struct TexturePackInfo {
    /// The UUID of the texture pack. Each texture pack downloaded must have a different UUID in
    /// order for the client to be able to handle them properly.
    pub uuid: String,
    /// The version of the texture pack. The client will cache texture packs sent by the server as
    /// long as they carry the same version. Sending a texture pack with a different version than
    /// previously will force the client to re-download it.
    pub version: String,
    /// The total size in bytes that the texture pack occupies. This is the size of the compressed
    /// archive (zip) of the texture pack.
    pub size: u64,
    /// The key used to decrypt the texture pack if it is encrypted. This is generally the case for
    /// marketplace texture packs.
    pub content_key: String,
    /// The purpose of this field is currently unknown.
    pub sub_pack_name: String,
    /// The purpose of this field is currently unknown.
    pub content_identity: String,
    /// Specifies if the texture packs has any scripts in it. A client will only download the
    /// texture pack if it supports scripts, which, up to 1.11, only includes Windows 10.
    pub has_scripts: bool,
    /// Specifies if the texture pack uses the ray-tracing technology introduced in 1.16.200.
    pub rtx_enabled: bool,
}
