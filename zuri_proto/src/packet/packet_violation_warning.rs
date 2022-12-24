use num_derive::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::Packet;

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

impl Packet for PacketViolationWarning {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.violation_type).unwrap());
        writer.var_i32(num::ToPrimitive::to_i32(&self.severity).unwrap());
        writer.var_i32(self.packet_id);
        writer.string(self.violation_context.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            violation_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            severity: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            packet_id: reader.var_i32(),
            violation_context: reader.string(),
        }
    }
}
