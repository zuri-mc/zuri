use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};
use crate::proto::types::resource_pack::ResourcePackResponse;

/// Sent by the client in response to resource packets sent by the server. It is used to let the server know what action
/// needs to be taken for the client to have all resource packs ready and set.
#[derive(Debug, Clone)]
pub struct ResourcePackClientResponse {
    /// The response type the client gave.
    pub response: ResourcePackResponse,
    /// A list of resource pack UUIDs combined with their version that need to be downloaded, if the Response field is
    /// PackResponseSendPacks.
    pub packs_to_download: Vec<String>,
}

impl PacketType for ResourcePackClientResponse {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.response.to_u8().unwrap());

        writer.u16(self.packs_to_download.len() as u16);
        self.packs_to_download.iter().for_each(|pack| writer.string(pack.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            response: ResourcePackResponse::from_u8(reader.u8()).unwrap(),
            packs_to_download: (0..reader.u16()).map(|_| reader.string()).collect(),
        }
    }
}
