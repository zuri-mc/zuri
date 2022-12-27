use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::world::ExperimentData;

/// Sent by the server to send the order in which resource packs and behaviour packs should be applied (and downloaded)
/// by the client.
#[derive(Debug, Clone)]
pub struct ResourcePackStack {
    /// Specifies if the client must accept the texture packs the server has in order to join the server. If set to
    /// true, the client gets the option to either download the resource packs and join, or quit entirely. Behaviour
    /// packs never have to be downloaded.
    pub texture_pack_required: bool,
    /// A list of behaviour packs that the client needs to download before joining the server. All of these behaviour
    /// packs will be applied together, and the order does not necessarily matter.
    pub behaviour_packs: Vec<StackResourcePack>,
    /// A list of texture packs that the client needs to download before joining the server. The order of these texture
    /// packs specifies the order that they are applied in on the client side. The first in the list will be applied
    /// before the rest.
    pub texture_packs: Vec<StackResourcePack>,
    /// The vanilla version that the client should set its resource pack stack to.
    pub base_game_version: String,
    /// A list of experiments that are either enabled or disabled in the world that the player spawns in. It is not
    /// clear why experiments are sent both here and in the StartGame packet.
    pub experiments: Vec<ExperimentData>,
    /// Specifies if any experiments were previously toggled in this world. It is probably used for metrics.
    pub experiments_previously_toggled: bool,
}

#[derive(Debug, Clone)]
pub struct StackResourcePack {
    pub uuid: String,
    pub version: String,
    pub sub_pack_name: String,
}

impl PacketType for ResourcePackStack {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.texture_pack_required);

        writer.var_u32(self.behaviour_packs.len() as u32);
        self.behaviour_packs.iter().for_each(|pack| pack.write(writer));

        writer.var_u32(self.texture_packs.len() as u32);
        self.texture_packs.iter().for_each(|pack| pack.write(writer));

        writer.string(self.base_game_version.as_str());

        writer.u32(self.experiments.len() as u32);
        self.experiments.iter().for_each(|experiment| experiment.write(writer));

        writer.bool(self.experiments_previously_toggled);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            texture_pack_required: reader.bool(),
            behaviour_packs: (0..reader.var_u32()).map(|_| StackResourcePack::read(reader)).collect(),
            texture_packs: (0..reader.var_u32()).map(|_| StackResourcePack::read(reader)).collect(),
            base_game_version: reader.string(),
            experiments: (0..reader.u32()).map(|_| ExperimentData::read(reader)).collect(),
            experiments_previously_toggled: reader.bool(),
        }
    }
}

impl StackResourcePack {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.string(self.version.as_str());
        writer.string(self.sub_pack_name.as_str());
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            version: reader.string(),
            sub_pack_name: reader.string(),
        }
    }
}
