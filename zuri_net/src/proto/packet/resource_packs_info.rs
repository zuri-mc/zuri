use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to inform the client on what resource packs the server has. It sends a list of the resource packs
/// it has and basic information on them like the version and description.
#[derive(Debug, Clone)]
pub struct ResourcePacksInfo {
    /// Specifies if the client must accept the texture packs the server has in order to join the server. If set to
    /// true, the client gets the option to either download the resource packs and join, or quit entirely. Behaviour
    /// packs never have to be downloaded.
    pub texture_pack_required: bool,
    /// Specifies if any of the resource packs contain scripts in them. If set to true, only clients that support
    /// scripts will be able to download them.
    pub has_scripts: bool,
    /// The use of this field is currently unknown.
    pub forcing_server_packs: bool,
    /// A list of behaviour packs that the client needs to download before joining the server. All of these behaviour
    /// packs will be applied together.
    pub behaviour_packs: Vec<BehaviourPackInfo>,
    /// A list of texture packs that the client needs to download before joining the server. The order of these texture
    /// packs is not relevant in this packet. It is however important in the ResourcePackStack packet.
    pub texture_packs: Vec<TexturePackInfo>,
}

#[derive(Debug, Clone)]
pub struct BehaviourPackInfo {
    pub uuid: String,
    pub version: String,
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
}

#[derive(Debug, Clone)]
pub struct TexturePackInfo {
    pub uuid: String,
    pub version: String,
    pub size: u64,
    pub content_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
    pub rtx_enabled: bool,
}

impl PacketType for ResourcePacksInfo {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.texture_pack_required);
        writer.bool(self.has_scripts);
        writer.bool(self.forcing_server_packs);

        writer.u16(self.behaviour_packs.len() as u16);
        self.behaviour_packs.iter().for_each(|pack| pack.write(writer));

        writer.u16(self.texture_packs.len() as u16);
        self.texture_packs.iter().for_each(|pack| pack.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            texture_pack_required: reader.bool(),
            has_scripts: reader.bool(),
            forcing_server_packs: reader.bool(),
            behaviour_packs: (0..reader.u16()).map(|_| BehaviourPackInfo::read(reader)).collect(),
            texture_packs: (0..reader.u16()).map(|_| TexturePackInfo::read(reader)).collect(),
        }
    }
}

impl BehaviourPackInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.string(self.version.as_str());
        writer.u64(self.size);
        writer.string(self.content_key.as_str());
        writer.string(self.sub_pack_name.as_str());
        writer.string(self.content_identity.as_str());
        writer.bool(self.has_scripts);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            version: reader.string(),
            size: reader.u64(),
            content_key: reader.string(),
            sub_pack_name: reader.string(),
            content_identity: reader.string(),
            has_scripts: reader.bool(),
        }
    }
}

impl TexturePackInfo {
    pub fn write(&self, writer: &mut Writer) {
        writer.string(self.uuid.as_str());
        writer.string(self.version.as_str());
        writer.u64(self.size);
        writer.string(self.content_key.as_str());
        writer.string(self.sub_pack_name.as_str());
        writer.string(self.content_identity.as_str());
        writer.bool(self.has_scripts);
        writer.bool(self.rtx_enabled);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            uuid: reader.string(),
            version: reader.string(),
            size: reader.u64(),
            content_key: reader.string(),
            sub_pack_name: reader.string(),
            content_identity: reader.string(),
            has_scripts: reader.bool(),
            rtx_enabled: reader.bool(),
        }
    }
}

