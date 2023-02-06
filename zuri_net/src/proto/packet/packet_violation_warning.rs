use zuri_net_derive::packet;
use crate::proto::ints::VarI32;

/// Sent by the client when it receives an invalid packet from the server. It holds some information
/// on the error that occurred.
#[packet]
#[derive(Debug, Clone)]
pub struct PacketViolationWarning {
    /// The type of violation.
    pub violation_type: PacketViolationType,
    /// Specifies the severity of the packet violation. The action the client takes after this
    /// violation depends on the severity sent.
    pub severity: PacketViolationSeverity,
    /// The ID of the invalid packet that was received.
    pub packet_id: VarI32,
    /// A description on the violation of the packet.
    pub violation_context: String,
}

#[packet(VarI32)]
#[derive(Debug, Clone)]
pub enum PacketViolationType {
    Malformed,
}

#[packet(VarI32)]
#[derive(Debug, Clone)]
pub enum PacketViolationSeverity {
    Warning,
    FinalWarning,
    TerminatingConnection,
}
