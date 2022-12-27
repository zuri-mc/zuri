use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug)]
pub struct PacketViolationWarning {
    pub violation_type: PacketViolationType,
    pub severity: PacketViolationSeverity,
    pub packet_id: i32,
    pub violation_context: String,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PacketViolationType {
    Malformed,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum PacketViolationSeverity {
    Warning,
    FinalWarning,
    TerminatingConnection,
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
