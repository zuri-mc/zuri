use zuri_net_derive::proto;

/// Education Edition packet sent by the server to open the URL to a Code Builder server.
#[proto]
#[derive(Debug, Clone)]
pub struct CodeBuilder {
    /// The URL to the Code Builder (WS) server.
    pub url: String,
    /// Specifies if the client should automatically open the Code Builder app. If set to true, the
    /// client will attempt to use the Code Builder app to connect to and interface with the server
    /// running at the URL above.
    pub should_open_code_builder: bool,
}
