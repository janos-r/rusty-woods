use crate::*;
use bevy_ecs_ldtk::utils::ldtk_pixel_coords_to_translation_pivoted;

#[derive(Clone, Component, Default)]
pub struct DoorRef {
    // for switching the level
    pub target_level_iid: LevelSelection,
    // for saving to the player where he wants to go
    // after the level and its target spawn, the target should realize the player is trying to reach it
    pub target_entity_iid: EntityIid,
}

#[derive(Component, Default)]
pub struct Door;

impl Door {
    const SPAWN_DISTANCE: f32 = 25.;
    fn spawn_offset(direction: &super::Direction) -> Vec2 {
        match direction {
            super::Direction::Up => Vec2::new(0., Self::SPAWN_DISTANCE),
            super::Direction::Right => Vec2::new(Self::SPAWN_DISTANCE, 0.),
            super::Direction::Down => Vec2::new(0., -Self::SPAWN_DISTANCE),
            super::Direction::Left => Vec2::new(-Self::SPAWN_DISTANCE, 0.),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct DoorBundle {
    door: Door,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    collider: Collider,
    #[with(super::with_collision_events)]
    active_events: ActiveEvents,
    #[from_entity_instance]
    linked_door: DoorRef,
    #[from_entity_instance]
    spawn_direction: super::Direction,
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

impl From<EntityInstance> for DoorRef {
    fn from(entity_instance: EntityInstance) -> Self {
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Entity_ref") else {
            return default();
        };
        let FieldValue::EntityRef(Some(entity_ref)) = &field_instance.value else {
            return default();
        };
        DoorRef {
            target_level_iid: LevelSelection::Iid(entity_ref.level_iid.clone()),
            // Known issue: It has to be this complicated, because I don't know if there is a way to fetch the target Point here.
            // The target (and its Point) spawns only after its level spawns.
            // Discussed in: https://github.com/Trouv/bevy_ecs_ldtk/discussions/113
            target_entity_iid: EntityIid(entity_ref.entity_iid.clone()),
        }
    }
}

pub fn spawn_door(
    query_door: Query<(&EntityInstance, &super::Direction), Added<Door>>,
    mut player_query: Query<(&EntityIid, &mut Transform, &DeriveZFromY), With<Player>>,
    level_query: Query<&Handle<LdtkLevel>>,
    levels: Res<Assets<LdtkLevel>>,
) {
    for (entity_instance, direction) in &query_door {
        if let Ok((player_target_iid, mut player_transform, dzfy)) = player_query.get_single_mut() {
            if player_target_iid.0 == entity_instance.iid {
                for level_handle in &level_query {
                    let level_px_hei = levels
                        .get(level_handle)
                        .expect("Level should be loaded")
                        .level
                        .px_hei;
                    let entity_size = IVec2::new(entity_instance.width, entity_instance.height);
                    // TODO: try just loading the translation here instead
                    let door_location = ldtk_pixel_coords_to_translation_pivoted(
                        entity_instance.px,
                        level_px_hei,
                        entity_size,
                        entity_instance.pivot,
                    );
                    // spawn close from the door, not on top
                    let spawn_location = door_location + Door::spawn_offset(direction);
                    player_transform.translation = Vec3::new(
                        spawn_location.x,
                        spawn_location.y,
                        dzfy.get(spawn_location.y),
                    )
                }
            }
        }
    }
}
