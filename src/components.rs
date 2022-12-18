pub mod door;
pub mod player;
pub mod sign;
pub mod ui;
pub mod wall;
pub use {door::*, player::*, sign::*, ui::*, wall::*};

use crate::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// Entity String ref from LDtk
#[derive(Clone, Component, Debug, Default, Deref, DerefMut)]
pub struct EntityIid(pub String);

impl From<EntityInstance> for EntityIid {
    fn from(entity_instance: EntityInstance) -> Self {
        EntityIid(entity_instance.iid)
    }
}

/// ←↑→↓
#[derive(Component, Debug, Default)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl From<EntityInstance> for Direction {
    fn from(entity_instance: EntityInstance) -> Self {
        let Some(field_instance) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == "Direction") else {
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

fn with_collision_events(_: EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}

#[derive(Component, Default)]
pub struct DeriveZFromY;

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
    pub fn get(y: f32) -> f32 {
        Self::MIRROR_BASE - y / Self::COEFFICIENT
    }
}

pub fn spawn_derive_z_from_y(
    mut query: Query<&mut Transform, (Added<Transform>, With<DeriveZFromY>)>,
) {
    for mut transform in &mut query {
        transform.translation.z = DeriveZFromY::get(transform.translation.y);
    }
}
