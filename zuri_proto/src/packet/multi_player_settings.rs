use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use crate::io::{Reader, Writer};
use crate::packet::PacketType;

#[derive(Debug)]
pub struct MultiPlayerSettings {
    pub action_type: MultiPlayerSettingsAction,
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum MultiPlayerSettingsAction {
    Enable,
    Disable,
    RefreshJoinCode,
}

impl PacketType for MultiPlayerSettings {
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.action_type.to_i32().unwrap());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action_type: MultiPlayerSettingsAction::from_i32(reader.var_i32()).unwrap(),
        }
    }
}
