use glam::Vec3;
use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::attribute::AttributeValue;
use crate::proto::types::world::EntityLink;

/// Sent by the server to the client to spawn an entity to the player. It is used for every entity except other players,
/// for which the AddPlayer packet is used.
#[derive(Debug, Clone)]
pub struct AddActor {
    /// The unique ID of the entity. The unique ID is a value that remains consistent across different sessions of the
    /// same world, but most servers simply fill the runtime ID of the entity out for this field.
    pub entity_unique_id: i64,
    /// The runtime ID of the entity. The runtime ID is unique for each world session, and entities are generally
    /// identified in packets using this runtime ID.
    pub entity_runtime_id: u64,
    /// The string entity type of the entity. A list of these entities may be found online.
    pub entity_type: String,
    /// The position to spawn the entity on. If the entity is on a distance that the player cannot see it, the entity
    /// will still show up if the player moves closer.
    pub position: Vec3,
    /// The initial velocity the entity spawns with. This velocity will initiate client side movement of the entity.
    pub velocity: Vec3,
    /// The vertical rotation of the entity. Facing straight forward yields a pitch of 0. Pitch is measured in degrees.
    pub pitch: f32,
    /// The horizontal rotation of the entity. Yaw is also measured in degrees.
    pub yaw: f32,
    /// The same as yaw, except that it applies specifically to the head of the entity. A different value for head_yaw
    /// than yaw means that the entity will have its head turned.
    pub head_yaw: f32,
    /// The same as yaw, except that it applies specifically to the body of the entity. A different value for body_yaw
    /// than head_yaw means that the entity will have its body turned, although it is unclear what the difference
    /// between body_yaw and yaw is.
    pub body_yaw: f32,
    /// A slice of attributes that the entity has. It includes attributes such as its health, movement speed, etc.
    pub attributes: Vec<AttributeValue>,
    /// A map of entity metadata, which includes flags and data properties that alter in particular the way the entity
    /// looks. Flags include ones such as 'on fire' and 'sprinting'. The meta values are indexed by their property key.
    // TODO: Implement entity metadata.
    // pub entity_metadata: dyn Any,
    /// A list of properties that the entity inhibits. These properties define specific attributes of the entity.
    // TODO: Implement entity properties.
    // pub entity_properties: dyn Any,
    /// A list of entity links that are currently active on the entity. These links alter the way the entity shows up
    /// when first spawned in terms of it shown as riding an entity. Setting these links is important for new viewers
    /// to see the entity is riding another entity.
    pub entity_links: Vec<EntityLink>,
}

impl PacketType for AddActor {
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.entity_unique_id);
        writer.var_u64(self.entity_runtime_id);
        writer.string(self.entity_type.as_str());

        writer.vec3(self.position);
        writer.vec3(self.velocity);

        writer.f32(self.pitch);
        writer.f32(self.yaw);
        writer.f32(self.head_yaw);
        writer.f32(self.body_yaw);

        writer.var_u32(self.attributes.len() as u32);
        self.attributes.iter().for_each(|attribute| attribute.write(writer));

        // TODO: Entity metadata.
        // TODO: Entity properties.

        writer.var_u32(self.entity_links.len() as u32);
        self.entity_links.iter().for_each(|link| link.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            entity_unique_id: reader.var_i64(),
            entity_runtime_id: reader.var_u64(),
            entity_type: reader.string(),

            position: reader.vec3(),
            velocity: reader.vec3(),

            pitch: reader.f32(),
            yaw: reader.f32(),
            head_yaw: reader.f32(),
            body_yaw: reader.f32(),

            attributes: (0..reader.var_u32()).map(|_| AttributeValue::read(reader)).collect(),

            // entity_metadata: {
            //     // TODO: Entity metadata.
            // },
            // entity_properties: {
            //     // TODO: Entity properties.
            // },

            //entity_links: (0..reader.var_u32()).map(|_| EntityLink::read(reader)).collect(),
            entity_links: vec![],
        }
    }
}
