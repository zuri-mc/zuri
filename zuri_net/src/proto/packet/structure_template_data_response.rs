use zuri_nbt::encoding::NetworkLittleEndian;
use zuri_net_derive::proto;

use crate::proto::io::NBT;
use crate::proto::types::structure::StructureTemplateDataRequestType;

/// Sent by the server to send data of a structure to the client in response to a
/// StructureTemplateDataRequest packet.
#[proto]
#[derive(Debug, Clone)]
pub struct StructureTemplateDataResponse {
    /// The name of the structure that was requested. This is the name used to export the structure
    /// to a file.
    pub structure_name: String,
    /// Contains NBT data of the structure template if a it was found by the StructureName that was
    /// sent in a StructureTemplateDataRequest packet.
    pub structure_template: Option<NBT<NetworkLittleEndian>>,
    /// The response type of the packet. This depends on the RequestType field sent in the
    /// StructureTemplateDataRequest packet.
    pub response_type: StructureTemplateDataRequestType,
}
