#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Ability {
    Build,
    Mine,
    DoorsAndSwitches,
    OpenContainers,
    AttackPlayers,
    AttackMobs,
    OperatorCommands,
    Teleport,
    Invulnerable,
    Flying,
    MayFly,
    InstantBuild,
    Lightning,
    FlySpeed,
    WalkSpeed,
    Muted,
    WorldBuilder,
    NoClip,
    Count,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AbilityLayerType {
    CustomCache,
    Base,
    Spectator,
    Commands,
    Editor,
}

#[derive(Debug)]
pub struct AbilityData {
    entity_unique_id: i64,
    player_permissions: u8,
    command_permission: u8,
    layers: Vec<AbilityLayer>,
}

impl AbilityData {
    pub fn write(&self, writer: &mut Writer) {
        writer.i64(self.entity_unique_id);
        writer.u8(self.player_permissions);
        writer.u8(self.command_permission);
        writer.u8(self.layers.len() as u8);
        self.layers.iter().for_each(|layer| layer.write(writer));
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.i64(),
            player_permissions: reader.u8(),
            command_permission: reader.u8(),
            layers: (0..reader.u8()).map(|_| AbilityLayer::read(reader)).collect(),
        }
    }
}

#[derive(Debug)]
pub struct AbilityLayer {
    layer_type: AbilityLayerType,
    abilities: u32,
    values: u32,
    fly_speed: f32,
    walk_speed: f32,
}

impl AbilityLayer {
    pub fn write(&self, writer: &mut Writer) {
        writer.u8(num::ToPrimitive::to_u8(&self.layer_type).unwrap());
        writer.u32(self.abilities);
        writer.u32(self.values);
        writer.f32(self.fly_speed);
        writer.f32(self.walk_speed);
    }

    pub fn read(reader: &mut Reader) -> Self {
        Self {
            layer_type: num::FromPrimitive::from_u8(reader.u8()).unwrap(),
            abilities: reader.u32(),
            values: reader.u32(),
            fly_speed: reader.f32(),
            walk_speed: reader.f32(),
        }
    }
}
