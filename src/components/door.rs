use crate::*;
use bevy_ecs_ldtk::utils::ldtk_grid_coords_to_translation;

use super::with_collision_events;

#[derive(Component, Default)]
pub struct Destination {
    pub level: String,
    pub coords: Vec3,
}

// TODO: not necessary?
// #[derive(Component, Default)]
// pub struct Door;

#[derive(Bundle, LdtkEntity)]
pub struct DoorBundle {
    // door: Door,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    destination: Destination,
    collider: Collider,
    #[with(with_collision_events)]
    active_events: ActiveEvents,
}

impl From<EntityInstance> for Destination {
    fn from(entity_instance: EntityInstance) -> Self {
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "DestinationLevel") else {
            return default();
        };
        let FieldValue::String(Some(destination_level)) = &field_instance.value else {
            return default();
        };
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "DestinationXY") else {
            return default();
        };
        let FieldValue::Point(Some(point)) = field_instance.value else {
            return default();
        };

        // TODO: take the grid_size and grid_height from the target level/layer
        let grid_height = 16;
        let grid_size = IVec2 { x: 16, y: 16 };

        let coords = ldtk_grid_coords_to_translation(point, grid_height, grid_size).extend(3.);
        Destination {
            level: destination_level.clone(),
            coords,
        }
    }
}
