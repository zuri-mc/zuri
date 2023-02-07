use zuri_net_derive::proto;

/// Sent by the server to the client. The packet is currently unused by both client and server.
#[proto]
#[derive(Debug, Clone)]
pub struct AddBehaviourTree {
    /// An unused string.
    pub behaviour_tree: String,
}
