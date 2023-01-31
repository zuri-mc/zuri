use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PacketViolationType {
    Malformed,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum PacketViolationSeverity {
    Warning,
    FinalWarning,
    TerminatingConnection,
}

/// Sent by the client when it receives an invalid packet from the server. It holds some information
/// on the error that occurred.
#[derive(Debug, Clone)]
pub struct PacketViolationWarning {
    /// The type of violation.
    pub violation_type: PacketViolationType,
    /// Specifies the severity of the packet violation. The action the client takes after this
    /// violation depends on the severity sent.
    pub severity: PacketViolationSeverity,
    /// The ID of the invalid packet that was received.
    pub packet_id: i32,
    /// A description on the violation of the packet.
    pub violation_context: String,
}

impl PacketType for PacketViolationWarning {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.violation_type.to_i32().unwrap());
        writer.var_i32(self.severity.to_i32().unwrap());
        writer.var_i32(self.packet_id);
        writer.string(self.violation_context.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            violation_type: PacketViolationType::from_i32(reader.var_i32()).unwrap(),
            severity: PacketViolationSeverity::from_i32(reader.var_i32()).unwrap(),
            packet_id: reader.var_i32(),
            violation_context: reader.string(),
        }
    }
}
