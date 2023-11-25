pub mod buildings;
pub mod door;
pub mod mobs;
pub mod player;
pub mod sign;
pub mod trees;
pub mod ui;
pub mod wall;
pub use {buildings::*, door::*, mobs::*, player::*, sign::*, trees::*, ui::*, wall::*};

use crate::*;

// Components - in this module, I keep only systems to spawn entities with their components.
// Systems that run on every tick I keep in the systems module.

/// ←↑→↓
#[derive(Component, Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl From<&EntityInstance> for Direction {
    fn from(entity_instance: &EntityInstance) -> Self {
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Direction")
        else {
            return default();
        };
        let FieldValue::Enum(Some(spawn_direction)) = &field_instance.value else {
            return default();
        };
        match spawn_direction.as_str() {
            "Up" => Direction::Up,
            "Right" => Direction::Right,
            "Down" => Direction::Down,
            "Left" => Direction::Left,
            _ => default(),
        }
    }
}

fn with_collision_events(_: &EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}

#[derive(Component, Default)]
pub struct DeriveZFromY {
    // visual_base_of_image
    px_below_center: f32,
}

impl DeriveZFromY {
    /*
    The default camera is on Z 1000, lets keep that.
    Dividing Y by 100 (the coefficient) and subtracting from the camera (max possible "mirror_base")
    at 1k would allow for a level up to 100k px vertically.
    Lets keep some room just for good measure. Subtracting from a 100 base gives space for up to 10k px.
    If a higher level (px) would be necessary, the coefficient can be also adjusted.
    btw the level and other entities spawn around Z 0-3.
    */
    const MIRROR_BASE: f32 = 100.;
    const COEFFICIENT: f32 = 100.;
    pub fn get(&self, y: f32) -> f32 {
        Self::MIRROR_BASE - (y - self.px_below_center) / Self::COEFFICIENT
    }

    pub fn spawn(mut query: Query<(&mut Transform, &DeriveZFromY), Added<DeriveZFromY>>) {
        for (mut transform, dzfy) in &mut query {
            transform.translation.z = dzfy.get(transform.translation.y);
        }
    }
}

impl From<i32> for DeriveZFromY {
    fn from(value: i32) -> Self {
        Self {
            px_below_center: value as f32,
        }
    }
}

// This "default" implementation works only if the sprite has its visual base exactly on the bottom of its tile.
// Otherwise, if there is some padding on the bottom of the tile, implement this number individually for that bundle. For example FrogBundle.
// This way you can use many different sorts of tiles with or without padding (space around the sprite).
impl From<&EntityInstance> for DeriveZFromY {
    fn from(entity_instance: &EntityInstance) -> Self {
        (entity_instance.height / 2).into()
    }
}
