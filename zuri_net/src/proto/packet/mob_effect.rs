use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MobEffectOperation {
    Add = 1,
    Modify,
    Remove,
}

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum MobEffectType {
    Speed = 1,
    Slowness,
    Haste,
    MiningFatigue,
    Strength,
    InstantHealth,
    InstantDamage,
    JumpBoost,
    Nausea,
    Regeneation,
    Resistance,
    FireResistance,
    WaterBreathing,
    Invisibility,
    Blindness,
    NightVision,
    Hunger,
    Weakness,
    Poison,
    Wither,
    HealthBoost,
    Absorption,
    Saturation,
    Levitation,
    FatalPoison,
    ConduitPower,
}

/// Sent by the server to apply an effect to the player, for example an effect like poison. It may
/// also be used to modify existing effects, or removing them completely.
#[derive(Debug, Clone)]
pub struct MobEffect {
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities
    /// are generally identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The operation of the packet, specifying the result client-side.
    pub operation: MobEffectOperation,
    /// The type of the effect to be added, removed or modified.
    pub effect_type: MobEffectType,
    /// The amplifier of the effect. Take note that the amplifier is not the same as the effect's
    /// level. The level is usually one higher than the amplifier, and the amplifier can be negative
    /// to reverse the behaviour effect.
    pub amplifier: i32,
    /// Specifies if viewers of the entity that gets the effect shows particles around it. If set to
    /// false, no particles are emitted around the entity.
    pub particles: bool,
    /// The duration of the effect in seconds. After the duration has elapsed, the effect will be
    /// removed automatically client-side.
    pub duration: i32,
}

impl PacketType for MobEffect {
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.entity_runtime_id);
        writer.u8(self.operation.to_u8().unwrap());
        writer.var_i32(self.effect_type.to_i32().unwrap());
        writer.var_i32(self.amplifier);
        writer.bool(self.particles);
        writer.var_i32(self.duration);
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_runtime_id: reader.var_u64(),
            operation: MobEffectOperation::from_u8(reader.u8()).unwrap(),
            effect_type: MobEffectType::from_i32(reader.var_i32()).unwrap(),
            amplifier: reader.var_i32(),
            particles: reader.bool(),
            duration: reader.var_i32(),
        }
    }
}
