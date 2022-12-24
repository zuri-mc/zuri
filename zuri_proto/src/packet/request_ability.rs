#[derive(Debug)]
pub struct RequestAbility {
    pub ability: Ability,
    //pub value: dyn Any,
}

impl Packet for RequestAbility {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(num::ToPrimitive::to_i32(&self.ability).unwrap());
        //writer.write_TODO(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            ability: num::FromPrimitive::from_i32(reader.var_i32()).unwrap(),
            //value: reader.read_TODO(),
        }
    }
}
