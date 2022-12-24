/// Sent by the server to damage the player's armour after being hit. The packet should never be used by servers as it
/// hands the responsibility over to the player completely, while the server can easily reliably update the armour
/// damage of players itself.
#[derive(Debug)]
pub struct HurtArmour {
    /// The cause of the damage dealt to the armour.
    pub cause: i32,
    /// The amount of damage points that was dealt to the player. The damage to the armour will be calculated by the
    /// client based upon this damage, and will also be based upon any enchantments that the armour may have.
    pub damage: i32,
    /// A bitset of all armour slots affected.
    pub armour_slots: i64,
}

impl Packet for HurtArmour {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.cause);
        writer.var_i32(self.damage);
        writer.var_i64(self.armour_slots);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            cause: reader.var_i32(),
            damage: reader.var_i32(),
            armour_slots: reader.var_i64(),
        }
    }
}
