/// Sent by the server to initiate an entity link client-side, meaning one entity will start riding another.
#[derive(Debug)]
pub struct SetActorLink {
    /// The link to be set client-side. It links two entities together, so that one entity rides another. Note that
    /// players that see those entities later will not see the link, unless it is also sent in the AddActor and
    /// AddPlayer packets.
    pub entity_link: EntityLink,
}

impl Packet for SetActorLink {
    fn write(&self, writer: &mut Writer) {
        self.entity_link.write(writer);
    }

    fn read(reader: &mut Reader) -> Self {
        Self { entity_link: EntityLink::read(reader) }
    }
}
