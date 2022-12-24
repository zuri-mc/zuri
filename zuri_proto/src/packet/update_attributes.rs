/// Sent by the server to update an amount of attributes of any entity in the world. These attributes include ones such
/// as the health or the movement speed of the entity.
#[derive(Debug)]
pub struct UpdateAttributes {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// A slice of new attributes that the entity gets. It includes attributes such as its health, movement speed, etc.
    /// Note that only changed attributes have to be sent in this packet. It is not required to send attributes that did
    /// not have their values changed.
    pub attributes: Vec<Attribute>,
    /// The server tick at which the packet was sent. It is used in relation to CorrectPlayerMovePrediction.
    pub tick: u64,
}

impl Packet for UpdateAttributes {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.var_u32(self.attributes.len() as u32);
        self.attributes.iter().for_each(|attribute| attribute.write(writer));
        writer.var_u64(self.tick);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            attributes: (0..reader.var_u32()).map(|_| Attribute::read(reader)).collect(),
            tick: reader.var_u64(),
        }
    }
}
