#[derive(Debug)]
pub struct SetPlayerGameType {
    pub game_type: GameType,
}

impl Packet for SetPlayerGameType {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.game_type).unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            game_type: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
        }
    }
}
