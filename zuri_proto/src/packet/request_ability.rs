use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::PacketType;
use crate::io::{Reader, Writer};
use crate::types::ability::Ability;

#[derive(Debug)]
pub struct RequestAbility {
    pub ability: Ability,
    //pub value: dyn Any, // TODO
}

impl PacketType for RequestAbility {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.ability.to_i32().unwrap());
        //writer.write_TODO(self.value);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            ability: Ability::from_i32(reader.var_i32()).unwrap(),
            //value: reader.read_TODO(),
        }
    }
}
