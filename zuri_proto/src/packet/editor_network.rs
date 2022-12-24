use crate::io::{Reader, Writer};
use crate::packet::Packet;

#[derive(Debug)]
pub struct EditorNetwork {
    //pub payload: dyn Any, // TODO: NBT
}

impl Packet for EditorNetwork {
    fn write(&self, writer: &mut Writer) {
        // TODO: NBT (payload)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            // payload: {
            //     // TODO: NBT
            // }
        }
    }
}
