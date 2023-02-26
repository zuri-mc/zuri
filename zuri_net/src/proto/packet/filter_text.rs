use zuri_net_derive::proto;

/// Sent by the both the client and the server. The client sends the packet to the server to allow
/// the server to filter the text server-side. The server then responds with the same packet and the
/// safer version of the text.
#[proto]
#[derive(Debug, Clone)]
pub struct FilterText {
    pub text: String,
    pub from_server: bool,
}
