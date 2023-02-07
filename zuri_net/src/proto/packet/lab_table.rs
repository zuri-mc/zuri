use zuri_net_derive::proto;

use crate::proto::io::BlockPos;

/// Sent by the client to let the server know it started a chemical reaction in Education Edition,
/// and is sent by the server to other clients to show the effects. The packet is only functional if
/// Education features are enabled.
#[proto]
#[derive(Debug, Clone)]
pub struct LabTable {
    /// The type of the action that was executed. Typically, only combine is sent by the client,
    /// whereas react is sent by the server.
    pub action_type: LabTableAction,
    /// The position at which the lab table used was located.
    pub position: BlockPos,
    /// The type of the reaction that took place as a result of the items put into the lab table.
    /// The reaction type can be either that of an item or a particle, depending on whatever the
    /// result was of the reaction.
    pub reaction_type: u8,
}

#[proto(u8)]
#[derive(Debug, Clone)]
pub enum LabTableAction {
    Combine,
    React,
    Reset,
}
