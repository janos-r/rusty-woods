use crate::*;

#[derive(Component, Default)]
pub struct Door;

impl Door {
    const SPAWN_DISTANCE: f32 = 10.;

    fn spawn_offset(direction: &super::Direction) -> Vec2 {
        match direction {
            super::Direction::Up => Vec2::new(0., Self::SPAWN_DISTANCE + Player::SHIFT_COLLIDER),
            super::Direction::Right => Vec2::new(Self::SPAWN_DISTANCE, 0.),
            super::Direction::Down => Vec2::new(0., -Self::SPAWN_DISTANCE),
            super::Direction::Left => Vec2::new(-Self::SPAWN_DISTANCE, 0.),
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn spawn(
        query_door: Query<(&EntityInstance, &super::Direction, &Transform), Added<Door>>,
        mut player_query: Query<(&EntityIid, &mut Transform), (With<Player>, Without<Door>)>,
    ) {
        for (entity_instance, direction, door_transform) in &query_door {
            if let Ok((player_target_iid, mut player_transform)) = player_query.get_single_mut() {
                if player_target_iid.0 == entity_instance.iid {
                    // spawn player close from the door, not on top
                    let offset = Door::spawn_offset(direction);
                    player_transform.translation.x = door_transform.translation.x + offset.x;
                    player_transform.translation.y = door_transform.translation.y + offset.y;
                    player_transform.translation.z = door_transform.translation.z;
                }
            }
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct DoorBundle {
    door: Door,
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

#[derive(Clone, Component, Default)]
pub struct DoorRef {
    // for switching the level
    pub target_level_iid: LevelSelection,
    // for saving to the player where he wants to go
    // after the level and its target spawn, the target should realize the player is trying to reach it
    pub target_entity_iid: EntityIid,
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
            // Known issue: It has to be this complicated, because I don't know if there is a way to fetch the target Point here.
            // The target (and its Point) spawns only after its level spawns.
            // Discussed in: https://github.com/Trouv/bevy_ecs_ldtk/discussions/113
            target_entity_iid: EntityIid(entity_ref.entity_iid.clone()),
            target_level_iid: LevelSelection::Iid(entity_ref.level_iid.clone()),
        }
    }
}
