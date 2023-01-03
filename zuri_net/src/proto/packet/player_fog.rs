use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to render the different fogs in the Stack. The types of fog are controlled by
/// resource packs to change how they are rendered, and the ability to create custom fog.
#[derive(Debug, Clone)]
pub struct PlayerFog {
    /// A list of fog identifiers to be sent to the client. Examples of fog identifiers are
    /// "minecraft:fog_ocean" and "minecraft:fog_hell".
    pub stack: Vec<String>,
}

impl PacketType for PlayerFog {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.stack.len() as u32);
        self.stack.iter().for_each(|stack| writer.string(stack.as_str()));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { stack: (0..reader.var_u32()).map(|_| reader.string()).collect() }
    }
}
