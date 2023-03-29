use crate::proto::ints::VarU32;
use zuri_net_derive::proto;

use crate::proto::types::command::SoftEnumAction;

/// Sent by the server to update a soft enum, also known as a dynamic enum, previously sent in the
/// AvailableCommands packet. It is sent whenever the enum should get new options or when some of
/// its options should be removed. The UpdateSoftEnum packet will apply for enums that have been set
/// in the AvailableCommands packet with the `Dynamic` field of the CommandEnum set to true.
#[proto]
#[derive(Debug, Clone)]
pub struct UpdateSoftEnum {
    /// The type of the enum. This type must be identical to the one set in the AvailableCommands
    /// packet, because the client uses this to recognise which enum to update.
    pub enum_type: String,
    /// A list of options that should be updated. Depending on the ActionType field, either these
    /// options will be added to the enum, the enum options will be set to these options or all of
    /// these options will be removed from the enum.
    #[len_type(VarU32)]
    pub options: Vec<String>,
    /// The type of the action to execute on the enum. The Options field has a different result,
    /// depending on what action type is used.
    pub action_type: SoftEnumAction,
}
