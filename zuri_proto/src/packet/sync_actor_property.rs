#[derive(Debug)]
pub struct SyncActorProperty {
    //pub property_data: dyn Any, // TODO: NBT
}

impl Packet for SyncActorProperty {
    fn write(&self, writer: &mut Writer) {
        // TODO: NBT (property_data)
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            // property_data: {
            //     // TODO: NBT
            // }
        }
    }
}
