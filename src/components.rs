pub mod door;
pub mod player;
pub mod sign;
pub mod ui;
pub use {door::*, player::*, sign::*, ui::*};

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
