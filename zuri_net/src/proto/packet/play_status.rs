use zuri_net_derive::packet;
use crate::proto::ints::I32BE;

/// Sent by the server to update a player on the play status. This includes failed statuses due to a
/// mismatched version, but also success statuses.
#[packet]
#[derive(Debug, Clone)]
pub struct PlayStatus {
    /// The status of the packet.
    pub status: PlayStatusType,
}

#[packet(I32BE)]
#[derive(Debug, Clone, PartialEq)]
pub enum PlayStatusType {
    LoginSuccess,
    LoginFailedClient,
    LoginFailedServer,
    PlayerSpawn,
    LoginFailedInvalidTenant,
    LoginFailedVanillaEdu,
    LoginFailedEduVanilla,
    LoginFailedServerFull,
    LoginFailedEditorVanilla,
    LoginFailedVanillaEditor,
}
