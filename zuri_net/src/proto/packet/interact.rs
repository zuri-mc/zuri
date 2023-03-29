use crate::proto::ints::VarU64;
use glam::Vec3;
use zuri_net_derive::proto;

/// Sent by the client when it interacts with another entity in some way. It used to be used for
/// normal entity and block interaction, but this is no longer the case now.
#[proto]
#[derive(Debug, Clone)]
pub struct Interact {
    /// The type of action that was executed by the player.
    pub action_type: InteractionAction,
}

#[proto(u8)]
#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionAction {
    LeaveVehicle(InteractionLeaveVehicle) = 3,
    MouseOverEntity(InteractionMouseOverEntity),
    NPCOpen(InteractionNPCOpen),
    OpenInventory(OpenInventory),
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionLeaveVehicle {
    /// The runtime ID of the entity that the player interacted with.
    pub target_entity_runtime_id: VarU64,
    /// The position that the player spawns at after leaving the vehicle.
    pub position: Vec3,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionMouseOverEntity {
    /// The runtime ID of the entity that the player interacted with.
    pub target_entity_runtime_id: VarU64,
    /// The position relative to the entity moused over over which the player hovered with its
    /// mouse/touch.
    pub position: Vec3,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionNPCOpen {
    /// The runtime ID of the entity that the player interacted with.
    pub target_entity_runtime_id: VarU64,
}

#[proto]
#[derive(Debug, Clone, PartialEq)]
pub struct OpenInventory {
    /// Unused.
    pub target_entity_runtime_id: VarU64,
}
