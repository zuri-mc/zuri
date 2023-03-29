use crate::proto::ints::VarI32;
use zuri_net_derive::proto;

/// Sent by the server to the client to inform the client of updated progress on a lesson. This
/// packet only functions on the Minecraft: Education Edition version of the game.
#[proto]
#[derive(Debug, Clone)]
pub struct LessonProgress {
    /// The action the client should perform to show progress.
    pub action: LessonAction,
    /// The score the client should use when displaying the progress.
    pub score: VarI32,
    /// The identifier of the lesson that is being progressed.
    pub identifier: String,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum LessonAction {
    Start,
    Complete,
    Restart,
}
