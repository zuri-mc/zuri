use glam::Vec3;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::packet::Packet;
use crate::io::{Reader, Writer};

/// Sent by the client when it interacts with another entity in some way. It used to be used for normal entity and block
/// interaction, but this is no longer the case now.
#[derive(Debug)]
pub struct Interact {
    /// The type of action that was executed by the player.
    pub action_type: InteractionAction,
    /// The runtime ID of the entity that the player interacted with. This is empty for the OpenInventory action type.
    pub target_entity_runtime_id: u64,
    /// Associated with the action type above. For the MouseOverEntity action, this is the position relative to the
    /// entity moused over over which the player hovered with its mouse/touch. For the LeaveVehicle, this is the
    /// position that the player spawns at after leaving the vehicle.
    pub position: Vec3,
}

impl Packet for Interact {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action_type.to_u8().unwrap());
        writer.var_u64(self.target_entity_runtime_id);
        match self.action_type {
            InteractionAction::MouseOverEntity | InteractionAction::LeaveVehicle => {
                writer.vec3(self.position);
            }
            _ => {}
        }
    }

    fn read(reader: &mut Reader) -> Self {
        let action_type = InteractionAction::from_u8(reader.u8()).unwrap();
        Self {
            action_type,
            target_entity_runtime_id: reader.var_u64(),
            position: if action_type == InteractionAction::MouseOverEntity || action_type == InteractionAction::LeaveVehicle {
                reader.vec3()
            } else {
                Vec3::default()
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum InteractionAction {
    LeaveVehicle = 3,
    MouseOverEntity = 4,
    NPCOpen = 5,
    OpenInventory = 6,
}
