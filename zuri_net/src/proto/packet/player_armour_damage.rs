use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

/// Sent by the server to damage the armour of a player. It is a very efficient packet, but
/// generally it's much easier to just send a slot update for the damaged armour.
#[derive(Debug, Clone)]
pub struct PlayerArmourDamage {
    /// A bitset of 4 bits that indicate which pieces of armour need to have damage dealt to them.
    /// The first bit, when toggled, is for a helmet, the second for the chestplate, the third for
    /// the leggings and the fourth for boots.
    pub bitset: u8,
    /// The amount of damage that should be dealt to the helmet.
    pub helmet_damage: i32,
    /// The amount of damage that should be dealt to the chestplate.
    pub chestplate_damage: i32,
    /// The amount of damage that should be dealt to the leggings.
    pub leggings_damage: i32,
    /// The amount of damage that should be dealt to the boots.
    pub boots_damage: i32,
}

impl PacketType for PlayerArmourDamage {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.bitset);
        if self.bitset & 0x01 != 0 {
            writer.i32(self.helmet_damage);
        }
        if self.bitset & 0x02 != 0 {
            writer.i32(self.chestplate_damage);
        }
        if self.bitset & 0x04 != 0 {
            writer.i32(self.leggings_damage);
        }
        if self.bitset & 0x08 != 0 {
            writer.i32(self.boots_damage);
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let bitset = reader.u8();
        Self {
            bitset,
            helmet_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
            chestplate_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
            leggings_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
            boots_damage: if bitset & 0x01 != 0 { reader.i32() } else { 0 },
        }
    }
}
