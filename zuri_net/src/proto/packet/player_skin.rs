use uuid::Uuid;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::skin::Skin;

/// Sent by the client to the server when it updates its own skin using the in-game skin picker. It
/// is relayed by the server, or sent if the server changes the skin of a player on its own accord.
/// Note that the packet can only be sent for players that are in the player list.
#[derive(Debug, Clone)]
pub struct PlayerSkin {
    /// The UUID of the player as sent in the Login packet when the client joined the server. It
    /// must match this UUID exactly for the skin to show up on the player.
    pub uuid: Uuid,
    /// The new skin to be applied on the player with the UUID in the field above. The skin,
    /// including its animations, will be shown after sending it.
    pub skin: Skin,
    /// No longer has a function. The field can be left empty at all times.
    pub new_skin_name: String,
    /// No longer has a function. The field can be left empty at all times.
    pub old_skin_name: String,
}

impl PacketType for PlayerSkin {
    fn write(&self, writer: &mut Writer) {
        writer.uuid(self.uuid);
        self.skin.write(writer);
        writer.string(self.new_skin_name.as_str());
        writer.string(self.old_skin_name.as_str());
        writer.bool(self.skin.trusted);
    }

    fn read(reader: &mut Reader) -> Self {
        let mut packet = Self {
            uuid: reader.uuid(),
            skin: Skin::read(reader),
            new_skin_name: reader.string(),
            old_skin_name: reader.string(),
        };
        packet.skin.trusted = reader.bool();

        packet
    }
}
