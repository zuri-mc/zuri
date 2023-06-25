use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_mod_billboard::prelude::*;

/// Gives all entities with the [Nametag] component a floating text object that follows them.
pub(super) struct NametagPlugin;

impl Plugin for NametagPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BillboardPlugin).add_systems((
            tag_create_system,
            tag_update_system,
            tag_remove_system,
        ));
    }
}

/// Gives an entity a floating nametag that hovers above the entity.
#[derive(Component, Clone)]
pub struct Nametag {
    /// The text to display on the nametag.
    pub contents: String,
    /// How far above the entity the nametag should be rendered.
    pub y_offset: f32,
}

/// Component used to track a nametag entity. Contains the id of the entity it is following.
#[derive(Component, Copy, Clone)]
struct NametagMarker;

/// Tracks the nametag entity associated with the entity.
#[derive(Component, Copy, Clone)]
#[component(storage = "SparseSet")]
struct NametagOwner(Entity);

fn tag_create_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    new_tags: Query<(Entity, &Nametag), Added<Nametag>>,
) {
    if new_tags.is_empty() {
        return;
    }

    let font = assets.load("monocraft.ttf");

    for (entity, new_tag) in &new_tags {
        let tag_entity = commands
            .spawn(BillboardTextBundle {
                text: Text::from_section(
                    new_tag.contents.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ),
                text_anchor: Anchor::BottomCenter,
                transform: Transform::from_xyz(0., new_tag.y_offset, 0.)
                    .with_scale(Vec3::splat(0.01)),
                ..Default::default()
            })
            .insert(NametagMarker)
            .id();

        commands
            .entity(entity)
            .insert(NametagOwner(tag_entity))
            .add_child(tag_entity);
    }
}

fn tag_update_system(
    update: Query<(&Nametag, &NametagOwner), Changed<Nametag>>,
    mut tags: Query<(&mut Text, &mut Transform), With<NametagMarker>>,
) {
    for (name_tag, owner) in &update {
        let tag_entity = tags.get_mut(owner.0);
        if tag_entity.is_err() {
            continue;
        }

        let (mut text, mut tag_transform) = tag_entity.unwrap();
        text.sections[0].value = name_tag.contents.clone();
        tag_transform.translation.y = name_tag.y_offset;
    }
}

fn tag_remove_system(
    mut commands: Commands,
    mut removed: RemovedComponents<Nametag>,
    query: Query<&NametagOwner>,
) {
    for (owner, name_tag_owner) in removed
        .iter()
        .filter_map(|v| query.get(v).ok().map(|comp| (v, comp)))
    {
        commands.entity(name_tag_owner.0).despawn_recursive();
        commands.entity(owner).remove::<NametagOwner>();
    }
}
