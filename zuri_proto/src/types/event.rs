encodable_enum!(
    #[derive(Debug)]
    pub enum EventType {
        AchievementAwardedEventType = 0,
        EntityInteractEventType = 1,
        PortalBuiltEventType = 2,
        PortalUsedEventType = 3,
        MobKilledEventType = 4,
        CauldronUsedEventType = 5,
        PlayerDiedEventType = 6,
        BossKilledEventType = 7,
        AgentCommandEventType = 8,
        AgentCreatedEventType = 9,
        PatternRemovedEventType = 10,
        SlashCommandExecutedEventType = 11,
        FishBucketedEventType = 12,
        MobBornEventType = 13,
        PetDiedEventType = 14,
        CauldronInteractEventType = 15,
        ComposterInteractEventType = 16,
        BellUsedEventType = 17,
        EntityDefinitionTriggerEventType = 18,
        RaidUpdateEventType = 19,
        MovementAnomalyEventType = 20,
        MovementCorrectedEventType = 21,
        //ExtractHoneyEventType = 22, todo
        //TargetBlockHitEventType = 23, todo
        //PiglinBarterEventType = 24, todo
        PlayerWaxedOrUnwaxedCopperEventType = 25,
        //CodeBuilderRuntimeActionEventType = 26, todo
        //CodeBuilderScoreboardEventType = 27, todo
        //StriderRiddenInLavaInOverworldEventType = 28, todo
        SneakCloseToSculkSensorEventType = 29,
    }
);

#[derive(Debug)]
pub struct BellUsedEventData {
    pub item_id: i32,
}

impl BellUsedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            item_id: reader.var_i32(),
        }
    }
}

impl EventData for BellUsedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.item_id);
    }

    fn event_type(&self) -> EventType {
        EventType::BellUsed
    }
}

#[derive(Debug)]
pub struct ExtractHoneyEventData {} // todo

impl ExtractHoneyEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct FishBucketedEventData {
    pub pattern: i32,
    pub preset: i32,
    pub bucketed_entity_type: i32,
    pub release: bool,
}

impl FishBucketedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            pattern: reader.var_i32(),
            preset: reader.var_i32(),
            bucketed_entity_type: reader.var_i32(),
            release: reader.bool(),
        }
    }
}

impl EventData for FishBucketedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.pattern);
        writer.var_i32(self.preset);
        writer.var_i32(self.bucketed_entity_type);
        writer.bool(self.release);
    }

    fn event_type(&self) -> EventType {
        EventType::FishBucketed
    }
}

impl MobBornEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            entity_type: reader.var_i32(),
            variant: reader.var_i32(),
            colour: reader.u8(),
        }
    }
}

impl EventData for MobBornEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.entity_type);
        writer.var_i32(self.variant);
        writer.u8(self.colour);
    }

    fn event_type(&self) -> EventType {
        EventType::AchievementAwarded
    }
}

#[derive(Debug)]
pub struct PlayerWaxedOrUnwaxedCopperEventData {}

impl PlayerWaxedOrUnwaxedCopperEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

impl EventData for PlayerWaxedOrUnwaxedCopperEventData {
    fn write(&self, _: &mut Writer) {}

    fn event_type(&self) -> EventType {
        EventType::ExtractHoney
    }
}

#[derive(Debug)]
pub struct PetDiedEventData {
    pub killed_by_owner: bool,
    pub killer_entity_unique_id: i64,
    pub pet_entity_unique_id: i64,
    pub entity_damage_cause: i32,
    pub pet_entity_type: i32,
}

impl PetDiedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            killed_by_owner: reader.bool(),
            killer_entity_unique_id: reader.var_i64(),
            pet_entity_unique_id: reader.var_i64(),
            entity_damage_cause: reader.var_i32(),
            pet_entity_type: reader.var_i32(),
        }
    }
}

impl EventData for PetDiedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.bool(self.killed_by_owner);
        writer.var_i64(self.killer_entity_unique_id);
        writer.var_i64(self.pet_entity_unique_id);
        writer.var_i32(self.entity_damage_cause);
        writer.var_i32(self.pet_entity_type);
    }

    fn event_type(&self) -> EventType {
        EventType::PetDied
    }
}

#[derive(Debug)]
pub struct PlayerDiedEventData {
    pub attacker_entity_id: i32,
    pub attacker_variant: i32,
    pub entity_damage_cause: i32,
    pub in_raid: bool,
}

impl PlayerDiedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            attacker_entity_id: reader.var_i32(),
            attacker_variant: reader.var_i32(),
            entity_damage_cause: reader.var_i32(),
            in_raid: reader.bool(),
        }
    }
}

impl EventData for PlayerDiedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.attacker_entity_id);
        writer.var_i32(self.attacker_variant);
        writer.var_i32(self.entity_damage_cause);
        writer.bool(self.in_raid);
    }

    fn event_type(&self) -> EventType {
        EventType::PlayerDied
    }
}

#[derive(Debug)]
pub struct PortalBuiltEventData {
    pub dimension_id: i32,
}

impl PortalBuiltEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            dimension_id: reader.var_i32(),
        }
    }
}

impl EventData for PortalBuiltEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.dimension_id);
    }

    fn event_type(&self) -> EventType {
        EventType::PortalBuilt
    }
}

#[derive(Debug)]
pub struct PortalUsedEventData {
    pub from_dimension_id: i32,
    pub to_dimension_id: i32,
}

impl PortalUsedEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            from_dimension_id: reader.var_i32(),
            to_dimension_id: reader.var_i32(),
        }
    }
}

impl EventData for PortalUsedEventData {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.from_dimension_id);
        writer.var_i32(self.to_dimension_id);
    }

    fn event_type(&self) -> EventType {
        EventType::PortalUsed
    }
}

#[derive(Debug)]
pub struct SneakCloseToSculkSensorEventData {}

impl SneakCloseToSculkSensorEventData {
    pub fn read(reader: &mut Reader) -> Self {
        Self {}
    }
}

impl EventData for SneakCloseToSculkSensorEventData {
    fn write(&self, _: &mut Writer) {}

    fn event_type(&self) -> EventType {
        EventType::ExtractHoney
    }
}
