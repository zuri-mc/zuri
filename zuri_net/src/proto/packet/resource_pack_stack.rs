use zuri_net_derive::proto;

use crate::proto::ints::VarU32;
use crate::proto::types::world::ExperimentData;

/// Sent by the server to send the order in which resource packs and behaviour packs should be
/// applied (and downloaded) by the client.
#[proto]
#[derive(Debug, Clone)]
pub struct ResourcePackStack {
    /// Specifies if the client must accept the texture packs the server has in order to join the
    /// server. If set to true, the client gets the option to either download the resource packs and
    /// join, or quit entirely. Behaviour packs never have to be downloaded.
    pub texture_pack_required: bool,
    /// A list of behaviour packs that the client needs to download before joining the server. All
    /// of these behaviour packs will be applied together, and the order does not matter.
    #[len_type(VarU32)]
    pub behaviour_packs: Vec<StackResourcePack>,
    /// A list of texture packs that the client needs to download before joining the server. The
    /// order of these texture packs specifies the order that they are applied in on the client
    /// side. The first in the list will be applied before the rest.
    #[len_type(VarU32)]
    pub texture_packs: Vec<StackResourcePack>,
    /// The vanilla version that the client should set its resource pack stack to.
    pub base_game_version: String,
    /// A list of experiments that are either enabled or disabled in the world that the player
    /// spawns in. It is not clear why experiments are sent both here and in the StartGame packet.
    #[len_type(u32)]
    pub experiments: Vec<ExperimentData>,
    /// Specifies if any experiments were previously toggled in this world. It is probably used for
    /// metrics.
    pub experiments_previously_toggled: bool,
}

/// Resource pack sent on the stack of the client. When sent, the client will apply them in the
/// order of the stack sent.
#[proto]
#[derive(Debug, Clone)]
pub struct StackResourcePack {
    /// The UUID of the resource pack. Each resource pack downloaded must have a different UUID in
    /// order for the client to be able to handle them properly.
    pub uuid: String,
    /// The version of the resource pack. The client will cache resource packs sent by the server as
    /// long as they carry the same version. Sending a resource pack with a different version than
    /// previously will force the client to re-download it.
    pub version: String,
    /// The purpose of this field is currently unknown.
    pub sub_pack_name: String,
}
