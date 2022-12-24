#[derive(Debug)]
pub struct PositionTrackingDBClientRequest {
    pub request_action: PositionTrackingDBRequestAction,
    pub tracking_id: i32,
}

impl Packet for PositionTrackingDBClientRequest {
    fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.request_action).unwrap());
        writer.var_i32(self.tracking_id);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            request_action: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            tracking_id: reader.var_i32(),
        }
    }
}
