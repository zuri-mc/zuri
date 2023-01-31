use zuri_net_derive::packet;

/// Sent by the server to the client. The packet is currently unused by both client and server.
#[packet]
#[derive(Debug, Clone)]
pub struct AddBehaviourTree {
    /// An unused string.
    pub behaviour_tree: String,
}
