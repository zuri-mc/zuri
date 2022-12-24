#[derive(Debug)]
pub struct AgentAction {
    pub identifier: String,
    pub action: AgentActionType,
    pub response: Bytes,
}

impl Packet for AgentAction {
    fn write(&self, writer: &mut Writer) {
        writer.string(self.identifier.as_str());
        writer.var_i32(num::ToPrimitive::to_i32(&self.action).unwrap());
        writer.byte_slice(&self.response);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            identifier: reader.string(),
            action: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            response: reader.byte_slice(),
        }
    }
}
