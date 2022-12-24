#[derive(Debug)]
pub struct PlayerArmourDamage {
    pub bitset: u8,
    pub helmet_damage: i32,
    pub chestplate_damage: i32,
    pub leggings_damage: i32,
    pub boots_damage: i32,
}

impl Packet for PlayerArmourDamage {
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
