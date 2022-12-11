pub mod door;
pub mod player;
pub mod sign;
pub mod ui;
pub use {door::*, player::*, sign::*, ui::*};

use crate::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn with_collision_events(_: EntityInstance) -> ActiveEvents {
    ActiveEvents::COLLISION_EVENTS
}
