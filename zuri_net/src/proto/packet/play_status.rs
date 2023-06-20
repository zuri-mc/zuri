use crate::proto::ints::I32BE;
use zuri_net_derive::proto;

/// Sent by the server to update a player on the play status. This includes failed statuses due to a
/// mismatched version, but also success statuses.
#[proto]
#[derive(Debug, Clone)]
pub struct PlayStatus {
    /// The status of the packet.
    pub status: PlayStatusType,
}

#[proto(I32BE)]
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
