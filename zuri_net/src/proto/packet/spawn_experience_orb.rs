use glam::Vec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to spawn an experience orb entity client-side. Much like the AddPainting
/// packet, it is one of the few packets that spawn an entity without using the AddActor packet.
#[derive(Debug, Clone)]
pub struct SpawnExperienceOrb {
    /// The position to spawn the experience orb on. If the entity is on a distance that the player
    /// cannot see it, the entity will still show up if the player moves closer.
    pub position: Vec3,
    /// The amount of experience in experience points that the orb carries. The client-side size of
    /// the orb depends on the amount of experience in the orb: There are 11 possible sizes for the
    /// orb, for 1–2, 3–6, 7–16, 17–36, 37–72, 73–148, 149–306, 307–616, 617–1236, 1237–2476, and
    /// 2477 and up.
    pub experience_amount: i32,
}

impl PacketType for SpawnExperienceOrb {
    fn write(&self, writer: &mut Writer) {
        writer.vec3(self.position);
        writer.var_i32(self.experience_amount);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            position: reader.vec3(),
            experience_amount: reader.var_i32(),
        }
    }
}
