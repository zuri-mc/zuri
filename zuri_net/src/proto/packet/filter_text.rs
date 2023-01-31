use zuri_net_derive::packet;

/// Sent by the both the client and the server. The client sends the packet to the server to allow
/// the server to filter the text server-side. The server then responds with the same packet and the
/// safer version of the text.
#[packet]
#[derive(Debug, Clone)]
pub struct FilterText {
    pub text: String,
    pub from_server: bool,
}
