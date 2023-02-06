use bytes::Bytes;
use zuri_net_derive::packet;

/// Education Edition packet sent by the client to run an operation with a code builder.
#[packet]
#[derive(Debug, Clone)]
pub struct CodeBuilderSource {
    /// The operation to be performed.
    pub operation: CodeBuilderOperation,
    /// The category in which the operation falls under.
    pub category: CodeBuilderCategory,
    /// Extra data about the operation performed. It is always empty unless the operation is set.
    pub value: Bytes,
}

#[packet(u8)]
#[derive(Debug, Clone)]
pub enum CodeBuilderCategory {
    None,
    Status,
    Instantiation,
}

#[packet(u8)]
#[derive(Debug, Clone)]
pub enum CodeBuilderOperation {
    None,
    Get,
    Set,
    Reset,
}
