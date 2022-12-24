encodable_enum!(
    #[derive(Debug)]
    pub enum StackRequestAction2 {
        TakeStackRequestAction = 0,
        PlaceStackRequestAction = 1,
        SwapStackRequestAction = 2,
        DropStackRequestAction = 3,
        DestroyStackRequestAction = 4,
        ConsumeStackRequestAction = 5,
        CreateStackRequestAction = 6,
        PlaceInContainerStackRequestAction = 7,
        TakeOutContainerStackRequestAction = 8,
        LabTableCombineStackRequestAction = 9,
        BeaconPaymentStackRequestAction = 10,
        MineBlockStackRequestAction = 11,
        CraftRecipeStackRequestAction = 12,
        AutoCraftRecipeStackRequestAction = 13,
        CraftCreativeStackRequestAction = 14,
        CraftRecipeOptionalStackRequestAction = 15,
        CraftGrindstoneRecipeStackRequestAction = 16,
        CraftLoomRecipeStackRequestAction = 17,
        CraftNonImplementedStackRequestAction = 18,
        CraftResultsDeprecatedStackRequestAction = 19,
    }
);

impl Default for StackRequestAction2 {
    fn default() -> Self {
        Self::CraftNonImplementedStackRequestAction(CraftNonImplementedStackRequestAction {})
    }
}

#[derive(Debug)]
pub struct TakeStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl TakeStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }
}

impl StackRequestAction for TakeStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Take
    }
}

#[derive(Debug)]
pub struct SwapStackRequestAction {
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl SwapStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }
}

impl StackRequestAction for SwapStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::Swap
    }
}

#[derive(Debug)]
pub struct TakeOutContainerStackRequestAction {
    pub count: u8,
    pub source: StackRequestSlotInfo,
    pub destination: StackRequestSlotInfo,
}

impl TakeOutContainerStackRequestAction {
    pub fn read(reader: &mut Reader) -> Self {
        Self {
            count: reader.u8(),
            source: StackRequestSlotInfo::read(reader),
            destination: StackRequestSlotInfo::read(reader),
        }
    }
}

impl StackRequestAction for TakeOutContainerStackRequestAction {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.count);
        self.source.write(writer);
        self.destination.write(writer);
    }

    fn action_type(&self) -> StackRequestActionType {
        StackRequestActionType::TakeOutContainer
    }
}
